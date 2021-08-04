# Pundit

Pundit is a note taking system which is very loosely based on the famous Zettelkasten method. The core of pundit is a simple set up which allows quickly creating new notes (a note is just a markdown/org type file with a title) and creating links between notes. 

There is a lot of software with very similar properties to pundit. One feature that sets pundit apart is that, it is a standalone program and not necessarily tied to any specific editor. Even though there currently only is an interface implementation for emacs, it would be fairly simple to implement an interface for a different editor, since all of the more complex logic (making sure the links in all notes are sound, resolving backlinks onto notes, sorting notes by the shortest link-path between them and other fun things) is done in pundit itself. 

Another feature is that pundit allows combining note taking and spaced repetition learning (using the amazing [Anki](https://ankisrs.net)). If at any time while writing a note, something is worth learning by heart, pundit can quickly add an entry into the current note (allowing the user to interactively choose a deck and note type if those aren't obvious from the context). 

A note containing an anki note could look like this:
![A sample pundit note with an anki note](/punditScreenshot1.png)

A script running in the background then automatically adds an anki note for this entry. Any changes to the entry in this note will also automatically be synchronized with the anki database. This makes it possible to create new anki notes quickly, without interrupting the workflow too much.

Pundit can also (very primitively, as of now) parse bibtex files and create notes corresponding to papers or other entries in the file
![A sample pundit note with an anki note](/punditScreenshot3.png)

Here, it automatically inserts a link to the paper note (to make it easy to find all notes about papers) and a citekey which can then be used to, for example, immediately open the associated pdf, using [org-ref](https://github.com/jkitchin/org-ref).
