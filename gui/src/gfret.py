#!/usr/bin/python

import os
import gi
import subprocess

gi.require_version("Gtk", "3.0")
from gi.repository import Gtk, GdkPixbuf, Gio

class GFretBoard(Gtk.Window):
    def __init__(self):
        Gtk.Window.__init__(self, title="GFret Fretboard Layout Tool")

        self.vbox = Gtk.VBox(spacing=0)
        self.add(self.vbox)

        self.scrollBox = Gtk.ScrolledWindow()
        self.scrollBox.set_policy(Gtk.PolicyType.AUTOMATIC, Gtk.PolicyType.NEVER)
        self.vbox.pack_start(self.scrollBox, True, True, 0)

        self.imagePreview = Gtk.Image()
        image = subprocess.run(["fblt", "-o", "-"], capture_output=True)
        stream = Gio.MemoryInputStream.new_from_data(image.stdout)
        pixbuf = GdkPixbuf.Pixbuf.new_from_stream_at_scale(stream, 800, -1, True)
        self.imagePreview.set_from_pixbuf(pixbuf)
        self.scrollBox.add(self.imagePreview)

        self.hboxScale = Gtk.Box(spacing = 6)
        self.vbox.pack_start(self.hboxScale, False, True, 0)

        labelScale = Gtk.Label(label = "Scale Length:")
        self.hboxScale.pack_start(labelScale, False, True, 0)

        adjustment = Gtk.Adjustment(upper=1000, lower=100, step_increment=0.1, page_increment=5)

        self.scale = Gtk.Scale(orientation=Gtk.Orientation.HORIZONTAL)
        self.scale.set_adjustment(Gtk.Adjustment(lower=100, upper=1000, step_increment=1, page_increment=10))
        self.scale.set_value(655)
        self.scale.set_draw_value(False)
        for i in range(100, 1100, 100):
            self.scale.add_mark(i, Gtk.PositionType.LEFT, str(i))
        self.hboxScale.pack_start(self.scale, True, True, 0)

        self.scaleFine = Gtk.SpinButton()
        self.scaleFine.set_adjustment(Gtk.Adjustment(upper=1000, lower=100, step_increment=0.1, page_increment=5))
        self.scaleFine.set_value(655)
        self.scaleFine.set_digits(1)
        self.hboxScale.pack_start(self.scaleFine, False, True, 0)

        self.hboxMulti = Gtk.Box(spacing = 6)
        self.vbox.pack_start(self.hboxMulti, False, True, 0)

        labelMulti = Gtk.Label(label = "Multiscale:")
        self.hboxMulti.pack_start(labelMulti, False, True, 0)

        self.checkBoxMulti = Gtk.CheckButton.new()
        self.hboxMulti.pack_start(self.checkBoxMulti, False, True, 0)

        self.scaleMulti = Gtk.HScale()
        self.scaleMulti.set_adjustment(Gtk.Adjustment(upper=1000, lower=100, step_increment=1, page_increment=10))
        self.scaleMulti.set_value(610)
        self.scaleMulti.set_draw_value(False)
        for i in range(100, 1100, 100):
            self.scaleMulti.add_mark(i, Gtk.PositionType.LEFT, str(i))
        self.scaleMulti.set_sensitive(False)
        self.hboxMulti.pack_start(self.scaleMulti, True, True, 0)

        self.scaleMultiFine = Gtk.SpinButton()
        self.scaleMultiFine.set_adjustment(Gtk.Adjustment(upper=1000, lower=100, step_increment=0.1, page_increment=5))
        self.scaleMultiFine.set_value(610)
        self.scaleMultiFine.set_digits(1)
        self.scaleMultiFine.set_sensitive(False)
        self.hboxMulti.pack_start(self.scaleMultiFine, False, True, 0)

        self.hboxSettings = Gtk.Box(spacing = 6)
        self.vbox.pack_start(self.hboxSettings, False, True, 0)

        self.vboxSettings0 = Gtk.VBox(spacing = 0)
        self.hboxSettings.pack_start(self.vboxSettings0, True, True, 0)

        self.hbox0 = Gtk.Box(spacing = 6)
        self.vboxSettings0.pack_start(self.hbox0, True, True, 0)

        self.fretsLabel = Gtk.Label(label = "Fret Count:")
        self.hbox0.pack_start(self.fretsLabel, True, True, 0)

        self.fretCount = Gtk.SpinButton()
        self.fretCount.set_adjustment(Gtk.Adjustment(upper=36, lower=8, step_increment=1, page_increment=4))
        self.fretCount.set_value(24)
        self.hbox0.pack_start(self.fretCount, False, True, 0)

        self.hbox1 = Gtk.Box(spacing = 6)
        self.vboxSettings0.pack_start(self.hbox1, True, True, 0)

        self.perpLabel = Gtk.Label(label = "Perpendicular Fret:")
        self.hbox1.pack_start(self.perpLabel, True, True, 0)

        self.perpFret = Gtk.SpinButton()
        self.perpFret.set_adjustment(Gtk.Adjustment(upper=12, lower=1, step_increment=1, page_increment=2))
        self.perpFret.set_value(8)
        self.hbox1.pack_start(self.perpFret, False, True, 0)

        self.vboxSettings1 = Gtk.VBox(spacing = 0)
        self.hboxSettings.pack_start(self.vboxSettings1, True, True, 0)

        self.hbox2 = Gtk.Box(spacing = 6)
        self.vboxSettings1.pack_start(self.hbox2, True, True, 0)

        self.nutLabel = Gtk.Label(label = "Nut Width:")
        self.hbox2.pack_start(self.nutLabel, True, True, 0)

        self.nut = Gtk.SpinButton()
        self.nut.set_adjustment(Gtk.Adjustment(upper=100, lower=20, step_increment=0.1, page_increment=2))
        self.nut.set_value(43)
        self.nut.set_digits(1)
        self.hbox2.pack_start(self.nut, False, True, 0)

        self.hbox3 = Gtk.Box(spacing = 6)
        self.vboxSettings1.pack_start(self.hbox3, True, True, 0)

        self.bridgeLabel = Gtk.Label(label = "Bridge Spacing:")
        self.hbox3.pack_start(self.bridgeLabel, True, True, 0)

        self.bridge = Gtk.SpinButton()
        self.bridge.set_adjustment(Gtk.Adjustment(upper=100, lower=20, step_increment=0.1, page_increment=2))
        self.bridge.set_value(56)
        self.bridge.set_digits(1)
        self.hbox3.pack_start(self.bridge, False, True, 0)

        self.vboxSettings2 = Gtk.VBox(spacing = 0)
        self.hboxSettings.pack_start(self.vboxSettings2, True, True, 0)

        self.hbox4 = Gtk.Box(spacing = 6)
        self.vboxSettings2.pack_start(self.hbox4, True, True, 0)

        self.borderLabel = Gtk.Label(label = "Border:")
        self.hbox4.pack_start(self.borderLabel, True, True, 0)

        self.border = Gtk.SpinButton()
        self.border.set_adjustment(Gtk.Adjustment(upper=20, lower=0, step_increment=1, page_increment=5))
        self.border.set_value(10)
        self.hbox4.pack_start(self.border, False, True, 0)

        self.hbox5 = Gtk.Box(spacing = 6)
        self.vboxSettings2.pack_start(self.hbox5, True, True, 0)

        self.outputLabel = Gtk.Label(label = "Output File:")
        self.hbox5.pack_start(self.outputLabel, True, True, 0)

        self.output = Gtk.Entry()
        self.output.set_text("output.svg")
        self.hbox5.pack_start(self.output, False, True, 0)

        self.hboxButtons = Gtk.Box(spacing=6)
        self.vbox.pack_start(self.hboxButtons, False, True, 0)

        labelExtern = Gtk.Label(label = "Open with:")
        self.hboxButtons.pack_start(labelExtern, False, True, 0)

        self.checkBoxExtern = Gtk.CheckButton.new()
        self.hboxButtons.pack_start(self.checkBoxExtern, False, True, 0)

        self.extern = Gtk.Entry()
        self.extern.set_text("inkscape")
        self.extern.set_sensitive(False)
        self.hboxButtons.pack_start(self.extern, False, True, 0)

        self.closebutton = Gtk.Button(label="Close")
        self.hboxButtons.pack_end(self.closebutton, False, True, 0)

        self.savebutton = Gtk.Button(label="Save")
        self.hboxButtons.pack_end(self.savebutton, False, True, 0)

        self.connect("configure-event", self.redraw_preview)
        self.scale.connect("value-changed", self.set_scale, self.scaleFine)
        self.scaleFine.connect("value-changed", self.set_scale, self.scale)
        self.checkBoxMulti.connect("toggled", self.toggle_widget, self.scaleMulti, self.scaleMultiFine)
        self.checkBoxMulti.connect("toggled", self.refresh_preview)
        self.scaleMultiFine.connect("value-changed", self.set_scale, self.scaleMulti)
        self.scaleMulti.connect("value-changed", self.set_scale, self.scaleMultiFine)
        self.fretCount.connect("value-changed", self.refresh_preview)
        self.perpFret.connect("value-changed", self.refresh_preview)
        self.nut.connect("value-changed", self.refresh_preview)
        self.bridge.connect("value-changed", self.refresh_preview)
        self.border.connect("value-changed", self.refresh_preview)
        self.checkBoxExtern.connect("toggled", self.toggle_widget, self.extern)
        self.closebutton.connect("clicked", Gtk.main_quit)
        self.savebutton.connect("clicked", self.save_image)

    def toggle_widget(self, button, *widgets):
        for widget in widgets:
            if button.get_active() == True:
                widget.set_sensitive(True)
            else:
                widget.set_sensitive(False)

    def set_scale(self, widget, *target):
        target[0].set_value(widget.get_value())
        self.refresh_preview(widget)

    def get_cmd(self):
        cmd = ["fblt"]
        cmd.append(str(self.scale.get_value()))
        if self.checkBoxMulti.get_active() == True:
            cmd.append("-m")
            cmd.append(str(self.scaleMulti.get_value()))
        cmd.append("-n")
        cmd.append(str(self.nut.get_value()))
        cmd.append("-b")
        cmd.append(str(self.bridge.get_value()))
        cmd.append("-p")
        cmd.append(str(int(self.perpFret.get_value())))
        cmd.append("-B")
        cmd.append(str(self.border.get_value()))
        cmd.append("-c")
        cmd.append(str(int(self.fretCount.get_value())))
        return cmd

    def redraw_preview(self, event, widget):
        self.refresh_preview(widget)

    def refresh_preview(self, widget):
        cmd = self.get_cmd()
        cmd.append("-o")
        cmd.append("-")

        image = subprocess.run(cmd, capture_output=True)
        stream = Gio.MemoryInputStream.new_from_data(image.stdout)
        width = self.get_size()[0]
        pixbuf = GdkPixbuf.Pixbuf.new_from_stream_at_scale(stream, width, -1, True)
        self.imagePreview.set_from_pixbuf(pixbuf)

    def save_image(self, widget):
        cmd = self.get_cmd()
        cmd.append("-o")
        cmd.append(self.output.get_text())
        if self.checkBoxExtern.get_active() == True:
            cmd.append("-e")
            cmd.append(self.extern.get_text())
        cmd = " ".join(cmd)
        os.system(cmd)

win = GFretBoard()
win.connect("destroy", Gtk.main_quit)
win.show_all()
Gtk.main()
