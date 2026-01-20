# MIVS - Multi-dimensional Intent Vector System

# That sounds pretty good but what is mivs?

> [!NOTE]
> **M**ulti-dimensional <br>
> **I**ntent <br>
> **V**ector <br>
> **S**ystem

So basically MIVS aims to create more realistic, reliable, and multi-dimensional dialogues.
Instead of forcing the player into fixed choices, the user can freely write their own text.

The system then analyzes this input and maps its meaning onto a dialogue state machine,
adjusting multiple intent and emotion values at the same time.

It’s like having invisible dialogue sliders (trust, fear, curiosity, hostility, etc.)
that slowly shift based on what the player says, rather than hard-switching between dialogue branches.

# Sample for the concept

**npc**  > Hello, how are you? <br>
**user** < i'm **fine**, **thanks** and you? <br>
...it's a "positive", "greeting" message. <br>
...the dialog helper chooses best return for this situation. <br>
**npc**  > good, thanks!