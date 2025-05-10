use crate::Character;
use escpos::{
    driver::Driver,
    errors::Result,
    printer_options::PrinterOptions,
    ui::line::{LineBuilder, LineStyle},
    utils::{JustifyMode, Protocol, UnderlineMode},
};
use log::info;
use text_splitter::TextSplitter;

trait PrinterExt {
    fn write_multiline(&mut self, s: &str) -> Result<&mut Self>;
}

impl<D: Driver> PrinterExt for escpos::printer::Printer<D> {
    fn write_multiline(&mut self, s: &str) -> Result<&mut Self> {
        let splitter = TextSplitter::new(self.options().get_characters_per_line() as usize);

        for s in splitter.chunks(s) {
            self.writeln(s)?;
        }

        Ok(self)
    }
}

pub(crate) struct Printer<D: Driver> {
    printer: escpos::printer::Printer<D>,
}

impl<D: Driver> Printer<D> {
    pub(crate) fn new(driver: D) -> Self {
        let mut printer = escpos::printer::Printer::new(
            driver,
            Protocol::default(),
            Some(PrinterOptions::default()),
        );

        info!("Initialise printer");
        printer.init().unwrap();

        let now = jiff::Zoned::now();
        printer
            .writeln(&format!("{now:.0}"))
            .unwrap()
            .writeln("llm-vn-host starting...")
            .unwrap()
            .feed()
            .unwrap()
            .print()
            .unwrap();

        Self { printer }
    }

    pub(crate) fn print_ready(
        &mut self,
        characters: &[Character],
        ollama_model_names: &[&str],
    ) -> Result<()> {
        let now = jiff::Zoned::now();

        // Set style
        self.printer
            .smoothing(true)?
            .justify(JustifyMode::LEFT)?
            .bold(false)?
            .underline(UnderlineMode::None)?
            .size(1, 1)?;

        // Print basic info
        self.printer
            .writeln(&format!("{now:.0}"))?
            .writeln("llm-vn-host")?
            .feed()?;

        // Print configured characters
        self.printer.writeln("Available characters:")?;
        for character in characters {
            self.printer
                .writeln(&format!(" - name: {}", character.name))?
                .writeln(&format!("   model: {}", character.model_name))?;
        }
        self.printer.feed()?;

        // Print Ollama models
        self.printer.writeln("Available Ollama models:")?;
        for model_name in ollama_model_names {
            self.printer.writeln(&format!(" - {model_name}"))?;
        }
        self.printer.feed()?;

        // Say we are ready
        self.printer.writeln("Ready!")?.print_cut()?;

        Ok(())
    }

    pub(crate) fn print_chat_header(&mut self, character: &Character) -> Result<()> {
        let line_style = LineBuilder::new().style(LineStyle::Simple).build();
        let now = jiff::Zoned::now();

        self.printer
            .size(1, 1)?
            .bold(false)?
            .underline(UnderlineMode::None)?
            .justify(JustifyMode::CENTER)?
            .writeln(&format!(
                "{:02}:{:02}:{:02}",
                now.hour(),
                now.minute(),
                now.second()
            ))?
            .feed()?
            .writeln("Chat with")?
            .size(2, 2)?
            .bold(true)?
            .underline(UnderlineMode::Single)?
            .writeln(&character.name)?
            .size(1, 1)?
            .bold(false)?
            .underline(UnderlineMode::None)?
            .feed()?
            .write_multiline(&character.description)?
            .feed()?
            .draw_line(line_style)?
            .feed()?
            .print()?;

        Ok(())
    }

    fn print_transcript_message(
        &mut self,
        justify: JustifyMode,
        name: &str,
        text: &str,
        feeds: usize,
    ) -> Result<()> {
        self.printer
            .justify(justify)?
            .bold(true)?
            .underline(UnderlineMode::Single)?
            .writeln(name)?
            .bold(false)?
            .underline(UnderlineMode::None)?
            .write_multiline(text)?
            .justify(JustifyMode::CENTER)?;

        for _ in 0..feeds {
            self.printer.writeln(".")?;
        }

        self.printer.print()?;

        Ok(())
    }

    pub(crate) fn print_user_message(&mut self, msg: &str) -> Result<()> {
        self.print_transcript_message(JustifyMode::LEFT, "You", msg, 1)
    }

    pub(crate) fn print_character_message(
        &mut self,
        character: &Character,
        msg: &str,
    ) -> Result<()> {
        self.print_transcript_message(JustifyMode::RIGHT, &character.name, msg, 6)
    }

    pub(crate) fn print_chat_footer(&mut self) -> Result<()> {
        let line_style = LineBuilder::new().style(LineStyle::Simple).build();

        self.printer
            .justify(JustifyMode::CENTER)?
            .bold(false)?
            .underline(UnderlineMode::None)?
            .size(1, 1)?
            .draw_line(line_style)?
            .feed()?
            .write_multiline("This chat was with a large language model. It may not accurately represent reality or the views of individuals. Do not blindly believe everything it has told you.")?
            .feed()?
            .write_multiline("Feel free to keep this print out.")?
            .feed()?
            .underline(UnderlineMode::Single)?
            .writeln("thelateshows.org.uk")?
            .writeln("makerspace.org.uk")?
            .writeln("github.com/DanNixon/llm-vn-lateshows25")?
            .underline(UnderlineMode::None)?
            .feed()?
            .print_cut()?;

        Ok(())
    }
}
