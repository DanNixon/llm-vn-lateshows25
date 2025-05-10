use crate::{
    display::{UpdateScreenSender, UPDATE_SCREEN},
    RpcResources,
};
use embassy_executor::Spawner;
use embassy_rp::{bind_interrupts, peripherals::USB};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_usb::UsbDevice;
use icd::{Screen, SetDisplay, ENDPOINT_LIST, TOPICS_IN_LIST, TOPICS_OUT_LIST};
use postcard_rpc::{
    define_dispatch,
    header::VarHeader,
    server::{
        impls::embassy_usb_v0_4::{
            dispatch_impl::{WireRxBuf, WireRxImpl, WireStorage, WireTxImpl},
            PacketBuffers,
        },
        Dispatch, Sender, Server,
    },
};
use static_cell::ConstStaticCell;

struct Context {
    screen_tx: UpdateScreenSender,
}

type AppDriver = embassy_rp::usb::Driver<'static, USB>;
type AppStorage = WireStorage<ThreadModeRawMutex, AppDriver, 256, 256, 64, 256>;
type BufStorage = PacketBuffers<1024, 1024>;
pub(crate) type AppTx = WireTxImpl<ThreadModeRawMutex, AppDriver>;
type AppRx = WireRxImpl<AppDriver>;
type AppServer = Server<AppTx, AppRx, WireRxBuf, MyApp>;

bind_interrupts!(pub struct Irqs {
    USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
});

static PBUFS: ConstStaticCell<BufStorage> = ConstStaticCell::new(BufStorage::new());
static STORAGE: AppStorage = AppStorage::new();

fn usb_config() -> embassy_usb::Config<'static> {
    let mut config = embassy_usb::Config::new(0x1209, 0x0001);
    config.manufacturer = Some("Dan Nixon");
    config.product = Some("llm-vn-controller");
    config.serial_number = Some("0");

    // Required for windows compatibility.
    // https://developer.nordicsemi.com/nRF_Connect_SDK/doc/1.9.1/kconfig/CONFIG_CDC_ACM_IAD.html#help
    config.device_class = 0xEF;
    config.device_sub_class = 0x02;
    config.device_protocol = 0x01;
    config.composite_with_iads = true;

    config
}

define_dispatch! {
    app: MyApp;
    spawn_fn: spawn_fn;
    tx_impl: AppTx;
    spawn_impl: postcard_rpc::server::impls::embassy_usb_v0_4::dispatch_impl::WireSpawnImpl;
    context: Context;

    endpoints: {
        list: ENDPOINT_LIST;

        | EndpointTy | kind  | handler             |
        | ---------- | ----- | ------------------- |
        | SetDisplay | async | set_display_handler |
    };
    topics_in: {
        list: TOPICS_IN_LIST;

        | TopicTy | kind | handler |
        | ------- | ---- | ------- |
    };
    topics_out: {
        list: TOPICS_OUT_LIST;
    };
}

async fn set_display_handler(context: &mut Context, _header: VarHeader, request: Screen) {
    context.screen_tx.send(request);
}

pub fn init(r: RpcResources, spawner: Spawner) -> Sender<AppTx> {
    let driver = embassy_rp::usb::Driver::new(r.usb, Irqs);
    let pbufs = PBUFS.take();
    let config = usb_config();

    let context = Context {
        screen_tx: UPDATE_SCREEN.sender(),
    };

    let (device, tx_impl, rx_impl) = STORAGE.init(driver, config, pbufs.tx_buf.as_mut_slice());
    let dispatcher = MyApp::new(context, spawner.into());
    let vkk = dispatcher.min_key_len();
    let server: AppServer = Server::new(
        tx_impl,
        rx_impl,
        pbufs.rx_buf.as_mut_slice(),
        dispatcher,
        vkk,
    );
    let sender = server.sender();

    spawner.must_spawn(usb_task(device));
    spawner.must_spawn(server_task(server));

    sender
}

#[embassy_executor::task]
async fn usb_task(mut usb: UsbDevice<'static, AppDriver>) {
    usb.run().await;
}

#[embassy_executor::task]
async fn server_task(mut server: AppServer) {
    loop {
        let _ = server.run().await;
    }
}
