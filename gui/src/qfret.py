#!/usr/bin/python

import os
import sys

from PyQt5.QtCore import Qt
from PyQt5 import QtSvg, QtWidgets
from PyQt5.QtWidgets import (QApplication, QCheckBox, QDialog,
    QDialogButtonBox, QFormLayout, QLabel, QHBoxLayout, QLineEdit,
    QSlider, QSpinBox, QVBoxLayout, QWidget)

class Dialog(QDialog):
    """Dialog."""
    def __init__(self, parent=None):
        """Initializer."""
        super().__init__(parent)
        self.setWindowTitle('Qfblt')

        self.vbox0 = QVBoxLayout()
        self.setLayout(self.vbox0)

        self.svgPreview = QtSvg.QSvgWidget('/tmp/qfret-preview.svg')
        self.svgPreview.renderer().setAspectRatioMode(Qt.KeepAspectRatio)
        self.vbox0.addWidget(self.svgPreview)

        self.hboxScale = QHBoxLayout()
        self.vbox0.addLayout(self.hboxScale)

        self.label0 = QLabel('Scale Length:')
        self.hboxScale.addWidget(self.label0, 1)

        self.scale = QSlider(Qt.Horizontal)
        self.scale.setRange(100, 1000)
        self.scale.setValue(650)
        self.scale.valueChanged.connect(lambda:self.set_scale(self.disp0, self.scale.value()))
        self.hboxScale.addWidget(self.scale, 100)

        self.disp0 = QSpinBox()
        self.disp0.setRange(100, 1000)
        self.disp0.setValue(650)
        self.disp0.valueChanged.connect(lambda:self.set_scale(self.scale, self.disp0.value()))
        self.hboxScale.addWidget(self.disp0, 1)

        self.hboxMulti = QHBoxLayout()
        self.vbox0.addLayout(self.hboxMulti)

        self.multiscale = QCheckBox('Multiscale:')
        self.multiscale.stateChanged.connect(lambda:self.toggle_widget(self.multiscale))
        self.multiscale.stateChanged.connect(self.refresh_preview)
        self.hboxMulti.addWidget(self.multiscale, 1)

        self.scale_treble = QSlider(Qt.Horizontal)
        self.scale_treble.setRange(100, 1000)
        self.scale_treble.setValue(610)
        self.scale_treble.setEnabled(False)
        self.scale_treble.valueChanged.connect(lambda:self.set_scale(self.disp1, self.scale_treble.value()))
        self.hboxMulti.addWidget(self.scale_treble, 100)

        self.disp1= QSpinBox()
        self.disp1.setRange(100, 1000)
        self.disp1.setValue(610)
        self.disp1.setEnabled(False)
        self.disp1.valueChanged.connect(lambda:self.set_scale(self.scale_treble, self.disp1.value()))
        self.hboxMulti.addWidget(self.disp1, 1)

        self.hboxForm = QHBoxLayout()
        self.vbox0.addLayout(self.hboxForm)

        self.formLeft = QFormLayout()
        self.hboxForm.addLayout(self.formLeft)

        self.frets = QSpinBox()
        self.frets.setRange(8, 36)
        self.frets.setValue(24)
        self.frets.valueChanged.connect(self.refresh_preview)
        self.formLeft.addRow('Fret Count:', self.frets)

        self.perp = QSpinBox()
        self.perp.setRange(0, 12)
        self.perp.setValue(8)
        self.perp.valueChanged.connect(self.refresh_preview)
        self.formLeft.addRow('Perpendiculat Fret', self.perp)

        self.formCenter = QFormLayout()
        self.hboxForm.addLayout(self.formCenter)

        self.nut = QSpinBox()
        self.nut.setRange(30, 100)
        self.nut.setValue(43)
        self.nut.valueChanged.connect(self.refresh_preview)
        self.formCenter.addRow('Nut Width:', self.nut)

        self.bridge = QSpinBox()
        self.bridge.setRange(30, 100)
        self.bridge.setValue(56)
        self.bridge.valueChanged.connect(self.refresh_preview)
        self.formCenter.addRow('Bridge Spacing:', self.bridge)

        self.formRight = QFormLayout()
        self.hboxForm.addLayout(self.formRight)

        self.border = QSpinBox()
        self.border.setRange(0, 100)
        self.border.setValue(10)
        self.border.valueChanged.connect(self.refresh_preview)
        self.formRight.addRow('Border:', self.border)

        self.output = QLineEdit()
        self.output.setText('output.svg')
        self.formRight.addRow('Output File:', self.output)

        self.hboxBtns = QHBoxLayout()
        self.vbox0.addLayout(self.hboxBtns)

        self.openfile = QCheckBox('Open With:')
        self.openfile.stateChanged.connect(lambda:self.toggle_widget(self.openfile))
        self.hboxBtns.addWidget(self.openfile, 1)

        self.external_program = QLineEdit()
        self.external_program.setText('inkscape')
        self.external_program.setEnabled(False)
        self.hboxBtns.addWidget(self.external_program, 100)

        self.hboxBtns.addStretch(100)

        self.btns = QDialogButtonBox()
        self.btns.setStandardButtons(
            QDialogButtonBox.Close | QDialogButtonBox.Save)
        self.btns.accepted.connect(self.save_image)
        self.btns.rejected.connect(self.close)
        self.hboxBtns.addWidget(self.btns)

        self.resize(1000, 300)

    def toggle_widget(self, button):
        if button == self.multiscale:
            widgets = [self.scale_treble, self.disp1]
        else:
            widgets = [self.external_program]
        for widget in widgets:
            if button.isChecked() == True:
                widget.setEnabled(True)
            else:
                widget.setEnabled(False)

    def set_scale(self, widget, value):
        widget.setValue(value)
        self.refresh_preview(widget)

    def get_cmd(self):
        cmd = ["fblt"]
        cmd.append(str(self.scale.value()))
        if self.multiscale.isChecked() == True:
            cmd.append("-m")
            cmd.append(str(self.scale_treble.value()))
        cmd.append("-n")
        cmd.append(str(self.nut.value()))
        cmd.append("-b")
        cmd.append(str(self.bridge.value()))
        cmd.append("-p")
        cmd.append(str(self.perp.value()))
        cmd.append("-B")
        cmd.append(str(self.border.value()))
        cmd.append("-c")
        cmd.append(str(self.frets.value()))
        return cmd

    def refresh_preview(self, widget):
        cmd = self.get_cmd()
        cmd.append("-o /tmp/qfret-preview.svg >/dev/null")
        cmd = " ".join(cmd)
        os.system(cmd)
        self.svgPreview.load('/tmp/qfret-preview.svg')
        self.svgPreview.renderer().setAspectRatioMode(Qt.KeepAspectRatio)

    def save_image(self):
        cmd = self.get_cmd()
        cmd.append("-o")
        cmd.append(self.output.text())
        if self.openfile.isChecked() == True:
            cmd.append("-e")
            cmd.append(self.external_program.text())
        cmd = " ".join(cmd)
        os.system(cmd)

os.system("fblt -o /tmp/qfret-preview.svg >/dev/null")
if __name__ == '__main__':
    app = QApplication(sys.argv)
    dlg = Dialog()
    dlg.show()
    sys.exit(app.exec_())
