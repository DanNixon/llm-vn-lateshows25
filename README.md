# LLM powered visual novels

A coming together of parts to make multiple randomly generated visual novel style text chats with a mix of fictional and non-fictional people.
Rapidly thrown together for [The Late Shows](https://thelateshows.org.uk/) 2025.

Transcripts are printed live on a till receipt printer (this entire project was completely not just an excuse to use the excessive inventory of receipt paper stashed in my spare room, honest).

The architecture is, in short:

- [Ollama](https://ollama.com/), with a set of custom models to provide the character side of the interaction (and also provide the choices for next things the user can say).
- An old EPSON receipt printer, provides a persistent transcript that can be kept as a souvenir as well as being the main way of seeing the character replies during the conversation.
- A controller with a TFT and some buttons, based on a [Peek-o-Display](https://github.com/dannixon/peek-o-display) in a 3D printed case, for user interaction.
- A [control program](./host-software/) to tie all of the above together.
