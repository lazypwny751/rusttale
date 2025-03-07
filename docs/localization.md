# rusttale localization.

Welcome to rusttale localization, before start you'll see the "locales" directory under the root, you need to write your transcripts under this directory.
And all the trasncriptions are in the binary not like i18n or something else, we're using for it [localize](https://github.com/PokeJofeJr4th/rust-localize) for now.
 
## Naming.
you can see there's predeclared files like "locales/part_one.rs" there, all the transcripts called under "locales/transcription.rs" it is the initial and caller. So if you want to add/call new transcription file go add something like:

```rs
// file: locales/transcription.rs
pub mod part_one;
pub mod my_part; // <- a new transcription example.
```
but be careful don't conflict them.

## Create a simple transcription.
For more details please look at the [localize](https://github.com/PokeJofeJr4th/rust-localize), but for this example the code like bellow will be enough for now.

```rs
use localize::localization_table; // this is always required.

localization_table! {
	SampleExample = LDSL {
	"hello world" = {
		en => "Hello World!",
		tr => "Merhaba Dünya!",
		fr => "Bonjour le Monde!",
	},
}}
```
So simple and very practical, good luck.
