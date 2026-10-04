# My design note for Àmì

I want the design system approved before I build a screen. I have no screen, component, or social preview yet. I am setting the visual rules first so the product feels like Àmì from the first pixel.

## My visual direction

I make Àmì feel quiet, editorial, and sure of itself. I borrow from the rhythm of a well-set language journal, then keep the interface warm and unmistakably digital. I avoid the spruce and copper palette, silhouettes, and visual language from Killowatt.

I pair a high-contrast serif display face with a clean system sans for reading and a system monospace for language codes, grapheme details, and ranking evidence. I use local system font stacks only, with no remote font request.

I use open corner marks, fine rules, generous whitespace, and uneven editorial columns in place of plain rectangle cards. I keep controls as underlined text or quiet, squared-off actions rather than generic pills. I give suggestions room to breathe without making the page feel like a dashboard.

## My color tokens

I build a new palette from deep plum ink, warm paper, iris, mineral blue, and citron. I keep citron as a small, deliberate accent instead of a large surface color.

| Token | Value | My use |
| --- | --- | --- |
| Night ink | `#211D35` | I use this for primary text and the darkest surfaces. |
| Soft paper | `#F8F5EE` | I use this for the page canvas and quiet reading surfaces. |
| Iris | `#8B83C8` | I use this for active language state and selected diacritic detail. |
| Mineral blue | `#6F8FA6` | I use this for secondary labels and calm supporting marks. |
| Citron | `#D5E58C` | I use this for a brief focus cue and the active suggestion edge. |
| Quiet line | `#D9D4E3` | I use this for dividers, brackets, and fine structure. |
| Muted ink | `#706B7C` | I use this for supporting copy after contrast checks. |

I measure every text pairing against WCAG 2.2 AA before I ship the page. I never rely on color alone to show focus, selection, or suggestion state.

## My interaction rules

I keep every suggestion optional. I show a candidate, a short reason, and a clear accept or dismiss action. I never replace a word without an explicit action.

I show Hausa hooked letters and Igbo subdots as nearby choices, not as a warning about the plain letter. I design the Pidgin surface to describe nearby spellings or possible phrasing, never to label a form wrong. I use no red underline, scolding copy, or correction badge for Pidgin.

I make keyboard navigation work from the first version. I give focus a visible outline, respect reduced-motion settings, preserve the caret and selection, and make each suggestion reachable without a pointer.

## My demo composition

I plan one focused writing surface with a quiet language switch, a large serif invitation, and a narrow monospace strip for language and corpus evidence. I separate regions with open rules and offset margins rather than cards. I keep the typed text and suggestions on the same page so the local-first promise stays obvious.

I have not drawn that screen yet. I will keep this note as the design source before any HTML, CSS, or social preview artwork exists.

## My references

I use the [WCAG 2.2 accessibility reference](https://www.w3.org/TR/WCAG22/) for contrast, keyboard access, and reduced-motion checks. I keep all type local so the demo does not need an external font service.
