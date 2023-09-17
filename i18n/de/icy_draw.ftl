font-editor-table = Zeichentabelle 0-{ $length }:

unsaved-title=Unbenannt

menu-file=Datei
menu-new=Neu…
menu-open=Datei öffnen…
menu-open_recent=Zuletzt geöffnet
menu-open_recent_clear=Liste leeren
menu-save=Speichern
menu-edit-sauce=SAUCE Info bearbeiten…
menu-set-canvas-size=Leinwandgröße ändern…
menu-close=Schließen
menu-save-as=Speichern unter…
menu-export=Exportieren…
menu-edit-font-outline=Font Outline…

menu-edit=Bearbeiten
menu-undo=Rückgängig
menu-redo=Wiederherstellen
menu-undo-op=Rückgängig: { $op }
menu-redo-op=Wiederherstellen: { $op }

menu-cut=Ausschneiden
menu-copy=Kopieren
menu-paste=Einfügen
menu-delete=Löschen
menu-rename=Umbenennen
menu-paste-as=Einfügen als
menu-paste-as-new-image=Neues Bild
menu-paste-as-brush=Neuer Pinsel
menu-erase=Löschen
menu-flipx=X Spiegeln
menu-flipy=Y Spiegeln
menu-justifyleft=Linksbündig
menu-justifyright=Rechtsbündig
menu-justifycenter=Zentrieren
menu-crop=Zuschneiden
menu-justify_line_center=Zeile zentrieren
menu-justify_line_left=Zeile linksbündig
menu-justify_line_right=Zeile rechtsbündig
menu-insert_row=Zeile einfügen
menu-delete_row=Zeile löschen
menu-insert_colum=Spalte einfügen
menu-delete_colum=Spalte löschen
menu-erase_row=Zeile leeren
menu-erase_row_to_start=Zeile bis Anfang leeren
menu-erase_row_to_end=Zeile bis Ende leeren
menu-erase_column=Spalte leeren
menu-erase_column_to_start=Spalte bis Anfang leeren
menu-erase_column_to_end=Spalte bis Ende leeren
menu-scroll_area_up=Hoch scrollen
menu-scroll_area_down=Runter scrollen
menu-scroll_area_left=Links scrollen
menu-scroll_area_right=Rechts scrollen
menu-mirror_mode=Spiegelmodus
menu-area_operations=Bereichsoperationen

menu-selection=Auswahl
menu-select-all=Alles auswählen
menu-select_nothing=Nichts
menu-inverse_selection=Invertieren

menu-colors=Farben
menu-color-mode=Farbmodus
menu-color-mode-unrestricted=Unbegrenzt
menu-color-mode-dos=16 Farben, blink
menu-color-mode-ice=16 Farben, ice
menu-color-mode-ext-colors=256 Farben
menu-color-mode-ext-font=16 Farben, ext font, blink
menu-color-mode-ext-font-ice=16 Farben, ext font, ice
menu-select_palette=Palette auswählen
menu-open_palette_directory=Öffne Palettenverzeichnis…

menu-view=Ansicht
menu-reference-image=Referenzbild setzen…
menu-toggle-reference-image=Referenzbild ein/ausblenden
menu-clear-reference-image=Löschen
menu-toggle_fullscreen=Vollbildmodus
menu-zoom=Vergrößerung ({$zoom})
menu-zoom_reset=Vergrößerung zurücksetzen
menu-zoom_in=Vergrößern
menu-zoom_out=Verkleinern
menu-guides=Hilfslinien
menu-raster=Gitter
menu-guides-off=Aus
menu-zoom-fit_size=Größe anpassen
menu-show_layer_borders=Ebenenrahmen anzeigen
menu-show_line_numbers=Zeilennummern anzeigen

menu-pick_attribute_under_caret=Attribut unter Cursor aufhebn
menu-default_color=Standardfarbe
menu-toggle_color=Farbe umschalten

menu-fonts=Fonts
menu-open_font_selector=Schriften…
menu-open_font_manager=Bufferfonts bearbeiten…
menu-open_font_directoy=Fontverzeichnis öffnen…
menu-open_tdf_directoy=TDF Verzeichnis öffnen…
menu-open_palettes_directoy=Palettenverzeichnis öffnen…

menu-help=Hilfe
menu-discuss=Diskussion
menu-open_log_file=Logdatei öffnen
menu-report-bug=Fehler melden
menu-about=Info…

tool-fg=Fg
tool-bg=Bg
tool-solid=Solid
tool-line=Line
tool-character=Zeichen
tool-shade=Schattieren
tool-colorize=Färben
tool-size-label=Größe:
tool-half-block=Halbblock
tool-custom-brush=Benutzerdefinierter Pinsel

tool-select-label=Auswahlmodus:
tool-select-normal=Rechteck
tool-select-character=Zeichen
tool-select-attribute=Attribute
tool-select-foreground=Vordergund
tool-select-background=Hintergrund
tool-select-description=Shift halten, um Auswahl hinzuzufügen. Control/Cmd zum Entfernen.

tool-fill-exact_match_label=Exakte Übereinstimmung

toolbar-new=Neu

new-file-title=Neue Datei
new-file-width=Breite
new-file-height=Höhe
new-file-ok=Ok
new-file-cancel=Abbrechen
new-file-create=Erstellen

edit-sauce-title=SAUCE Info
edit-sauce-title-label=Titel:
edit-sauce-title-label-length=(35 Zeichen)
edit-sauce-author-label=Autor:
edit-sauce-author-label-length=(20 Zeichen)
edit-sauce-group-label=Grupe:
edit-sauce-group-label-length=(20 Zeichen)
edit-sauce-comments-label=Kommentare:
edit-sauce-letter-spacing=Zeichenabstand:
edit-sauce-aspect-ratio=Seitenverhältnis:

edit-canvas-size-title=Leinwandgröße
edit-canvas-size-width-label=Breite:
edit-canvas-size-height-label=Höhe:
edit-canvas-size-resize=Größe ändern
edit-canvas-size-resize_layers-label=Ebenen anpassen

toolbar-size = {$colums ->
     [1] 1 Zeile
*[other] {$colums} Zeilen
} x { $rows ->
     [1] 1 Spalte
*[other] { $rows } Spalten
}

toolbar-position = Zeile { $line }, Spalte { $column }

add_layer_tooltip = Neue Ebene
move_layer_up_tooltip = Ebene hoch
move_layer_down_tooltip = Ebene runter
delete_layer_tooltip = Ebene löschen
anchor_layer_tooltip = Ebene verankern

glyph-char-label=Zeichen
glyph-font-label=Schriftart

color-is_blinking=Blinken

export-title=Export
export-button-title=Export
export-file-label=Datei:
export-video-preparation-label=Video Vorbereitung:
export-video-preparation-None=Keine
export-video-preparation-Clear=Bildschirm löschen
export-video-preparation-Home=Cursor zurücksetzen
export-utf8-output-label=UTF-8 Ausgabe
export-save-sauce-label=SAUCE Info speichern
export-compression-level-label=Kompressionsstufe
export-compression-level-off=Keine
export-compression-level-medium=Mittel
export-compression-level-high=Hoch
export-limit-output-line-length-label=Maximale Ausgabe-Zeilenlänge
export-maximum_line_length=Maximale Zeilenlänge

select-character-title=Zeichen auswählen

select-outline-style-title=Outline Stil auswählen

about-dialog-title=Über Icy Draw
about-dialog-heading = Icy Draw
about-dialog-description = 
    Icy Draw ist ein Tool, um Ansis & Asciis zu erstellen.
    Entwickelt wurde es mit Rust und egui.

    Icy Draw is freie Software unter der Apache 2 Lizens.
    Die Homepage ist unter www.github.com/mkrueger/icy_draw
about-dialog-created_by = Erstellt von { $authors }

edit-layer-dialog-title=Ebene bearbeiten
edit-layer-dialog-name-label=Name:
edit-layer-dialog-is-visible-checkbox=Sichtbar
edit-layer-dialog-is-edit-locked-checkbox=Edit gesperrt
edit-layer-dialog-is-position-locked-checkbox=Position gesperrt
edit-layer-dialog-is-x-offset-label=X Versatz:
edit-layer-dialog-is-y-offset-label=Y Versatz:
edit-layer-dialog-has-alpha-checkbox=Hat Alphakanal
edit-layer-dialog-is-alpha-locked-checkbox=Alphakanal gesperrt

error-load-file=Fehler während des Dateiladens: { $error }

select-font-dialog-title=Font auswählen ({ $fontcount} verfügbar)
select-font-dialog-select=Auswählen
select-font-dialog-filter-text=Filter
select-font-dialog-no-fonts=Keine Fonts gefunden
select-font-dialog-no-fonts-installed=Keine Fonts installiert
select-font-dialog-color-font=FARBE
select-font-dialog-block-font=BLOCK
select-font-dialog-outline-font=OUTLINE
select-font-dialog-preview-text=HALLO
select-font-dialog-edit-button=Font bearbeiten…

layer_tool_title=Ebenen
layer_tool_menu_layer_properties=Ebeneneigenschaften…
layer_tool_menu_resize_layer=Ebenengröße…
layer_tool_menu_new_layer=Neue Ebene
layer_tool_menu_duplicate_layer=Duplizieren
layer_tool_menu_merge_layer=Ebene zusammenführen
layer_tool_menu_delete_layer=Ebene löschen
layer_tool_menu_clear_layer=Ebene leeren

channel_tool_title=Kanäle
channel_tool_fg=Vordergrund
channel_tool_bg=Hintergrund

font_tool_select_outline_button=Outline Stil auswählen
font_tool_current_font_label=Aktueller TDF Font
font_tool_no_font=<nichts>
font_tool_no_fonts_label=
    Keine Fonts gefunden
    Installiere Fonts in das Fontverzeichnis
font_tool_open_directory_button=Fontverzeichnis öffnen

char_table_tool_title=Zeichentabelle
minimap_tool_title=Übersicht

no_document_selected=Kein Dokument ausgewählt

undo-draw-ellipse=Ellipse
undo-draw-rectangle=Rechteck
undo-paint-brush=Pinsel
undo-pencil=Stift
undo-eraser=Radierer
undo-bucket-fill=Füllen
undo-line=Linie
undo-cut=Ausschneiden
undo-paste-glyph=Zeichen einfügen
undo-bitfont-flip-y=Y spiegeln
undo-bitfont-flip-x=X spiegeln
undo-bitfont-move-down=Runter
undo-bitfont-move-up=Hoch
undo-bitfont-move-left=Links
undo-bitfont-move-right=Rechts
undo-bitfont-inverse=Invertieren
undo-bitfont-clear=Leeren
undo-bitfont-edit=Editieren
undo-render_character=Zeichen rendern
undo-delete_character=Zeichen löschen
undo-select=Auswahl

autosave-dialog-title=Autosave
autosave-dialog-description=Icy Draw hat eine Autosave Datei gefunden.
autosave-dialog-question=Was möchtest du tun?
autosave-dialog-load_autosave_button=Autosave laden
autosave-dialog-discard_autosave_button=Verwerfen

paste_mode-description=Einfügemodos. Im Layer Tool neue Ebene hinzufügen oder verankern.
paste_mode-stamp=Stempel
paste_mode-rotate=Rotieren
paste_mode-flipx=X Spiegeln
paste_mode-flipy=Y Spiegeln
paste_mode-transparent=Transparent

ask_close_file_dialog-description=Sollen die Änderungen in { $filename } gespeichert werden?
ask_close_file_dialog-subdescription=Alle Änderungen gehen beim Verwerfen verloren.
ask_close_file_dialog-dont_save_button=Verwerfen
ask_close_file_dialog-save_button=Speichern

tab-context-menu-close=Schließen
tab-context-menu-close_others=Andere schließen
tab-context-menu-close_all=Alle schließen
tab-context-menu-copy_path=Pfad kopieren

font-view-char_label=Zeichen
font-view-ascii_label=ASCII
font-view-font_label=Font

font-editor-tile_area=Kacheln
font-editor-clear=Löschen
font-editor-inverse=Invertieren
font-editor-flip_x=X Spiegeln
font-editor-flip_y=Y Spiegeln

animation_editor_path_label=Datei:
animation_editor_export_button=Export
animation_editor_gif_label=Gif
animation_editor_ansi_label=Ansimation

new-file-template-ansi-title=CP437 ANSI
new-file-template-ansi-description=Ein neues DOS Ansi erstellen
new-file-template-file_id-title=FILE_ID.DIZ
new-file-template-file_id-description=FILE_ID.DIZ erstellen
new-file-template-ansimation-title=Ansimation
new-file-template-ansimation-description=Eine Ansi-Animation erstellen
new-file-template-bit_font-title=Bit Font
new-file-template-bit_font-description=Einen neuen Bit-Font erstellen
new-file-template-color_font-title=TDF Farb-Font
new-file-template-color_font-description=Einen neuen TDF Farb-Font erstellen
new-file-template-block_font-title=TDF Block-Font
new-file-template-block_font-description=Einen neuen TDF Block-Font erstellen
new-file-template-outline_font-title=TDF Outline-font
new-file-template-outline_font-description=Einen neuen TDF Outline-Font erstellen
new-file-template-ansimation-ui-label=
    Eine IcyDraw Animation ist eine Lua Textdatei, die eine Animationssequenz beschreibt.
    Für eine vollständige Beschreibung der Syntax klicke auf diesen Link:
new-file-template-bitfont-ui-label=
    Ein Bit-Font ist eine Sammlung von Zeichen, die in einem Raster angeordnet sind.
    Bit-Fonts können in ANSI-Editoren verwendet werden.

new-file-template-thedraw-ui-label=
    TheDraw Fonts können in ANSI-Editoren verwendet werden, um größere Zeichen zu definieren.
    TheDraw definiert drei Font-Typen: Farbe, Block und Outline.

    Eine große Font-Sammlung kann hier heruntergeladen werden:

palette_selector-dos_default_palette=VGA 16 Farben
palette_selector-c64_default_palette=C64 Farben
palette_selector-ega_default_palette=EGA 64 Farben
palette_selector-xterm_default_palette=XTerm erweiterte Palette
palette_selector-viewdata_default_palette=Viewdata
palette_selector-extracted_from_buffer_default_label=Erzeugt aus offener Datei