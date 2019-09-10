# Bezier Editor

This example is a barebones bezier path editor.

The editor has two tools, the select (arrow) tool, and the curve (pen) tool.

The tools can be selected via the menu, or by the standard hotkeys 'V' and 'P'.

## Select tool

The select tool can be used to select and move points.

In addition, several other keyboard commands are available when select is
active:

cmd+G adds a new guideline
cmd+A selects all points
tab selects the next point
shift+tab selects the previous point
backspace deletes the current selection.


click selects points. Shift + click toggles points in the current selection
group.

double click on an on-curve point toggles it between a corner point and a smooth
or tangent point.

double clicking on a guide toggles it between vertical and horizontal.

double clicking on an outline selects that outline.

## Pen

The pen tool adds points to an outline. If no point is currently selected, or
a point is selected in a closed path, clicking starts a new path. If a point is
selected in an open path, clicking appeends a point to that path.

shift+click constrains the new point to be verticall or horizontally aligned
with the previous point; whichever is the lesser distance.

double-click adds a point and deselects the current outline.

clicking on the start point of the active outline closes the outline.

click+drag modifies the off-curve (handle) points.

## Missing features

This is intended as a very rough proof of concept, and numerous features are
missing, such as undo, saving, or opening previously saved points.
