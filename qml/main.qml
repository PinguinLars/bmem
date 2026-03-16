/*
 * Bmem: a memory game
 * Copyright (C) 2026 AshyPinguin
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Window 2.12
import QtQuick.Layouts 2.12

import me.ashypinguin.bmem 1.0

ApplicationWindow {
  id: root
  title: qsTr("Biologie Memory")
  visible: true
  color: palette.window

  readonly property Deck deck: Deck {}

  RowLayout {
    anchors.fill: parent
    spacing: 3

    GridLayout {
      rows: 6 
      columns: 7
      Layout.fillHeight: true
      Layout.preferredWidth: parent.width * 0.8
      columnSpacing: 5
      rowSpacing: 5

      Repeater {
        model: root.deck.number_of_cards

        Rectangle {
          Layout.fillWidth: true
          Layout.fillHeight: true
          color: ma.containsMouse ? "#607D8B" : "#9E9E9E"
          radius: 10

          MouseArea {
            id: ma
            anchors.fill: parent
            hoverEnabled: true
            onClicked: root.deck.handleClickEvent(index)
          }

          Text {
            anchors.fill: parent
            text: root.deck.getCardText(index)

            horizontalAlignment: Text.AlignHCenter
            verticalAlignment: Text.AlignVCenter
            wrapMode: Text.WordWrap
          }
        }
      }
    }

    ColumnLayout {
      Layout.fillHeight: true
      Layout.preferredWidth: parent.width * 0.2

      Rectangle {
        Layout.preferredWidth: diffText.implicitWidth
        Layout.preferredHeight: diffText.implicitHeight + 30
        Layout.fillWidth: true

        topLeftRadius: 20
        topRightRadius: 20
        bottomLeftRadius: 0
        bottomRightRadius: 0

        Text {
          id: scoreText

          anchors.fill: parent
          anchors.leftMargin: 5
          anchors.rightMargin: 5
          anchors.topMargin: 10
          anchors.bottomMargin: 5
          horizontalAlignment: Text.AlignLeft

          text: `Your score: ${root.deck.number_of_cards/2}/${root.deck.number_of_cards/2}\nComputer score: 0/${root.deck.number_of_cards/2}`
        }
      }

      Rectangle {
        Layout.preferredWidth: turnText.implicitWidth
        Layout.preferredHeight: turnText.implicitHeight + 10
        Layout.fillWidth: true

        Text {
          id: turnText

          anchors.fill: parent
          anchors.margins: 5
          horizontalAlignment: Text.AlignLeft

          text: `Turn: ${root.deck.your_turn ? "you" : "computer"}` // or you
        }
      }

      Rectangle {
        Layout.preferredWidth: diffText.implicitWidth
        Layout.preferredHeight: diffText.implicitHeight + 15
        Layout.fillWidth: true

        topLeftRadius: 0
        topRightRadius: 0
        bottomLeftRadius: 20
        bottomRightRadius: 20

        Text {
          id: diffText

          anchors.fill: parent
          anchors.leftMargin: 5
          anchors.rightMargin: 5
          anchors.topMargin: 5
          anchors.bottomMargin: 10
          horizontalAlignment: Text.AlignLeft

          text: "Moeilijkheids: 50%"
        }
      }

      Item { Layout.fillHeight: true }

      Text {
        text: "Biologie memory (bmem) is free software licensed under the GPLv3 (or later) license"

        Layout.fillWidth: true
        horizontalAlignment: Text.AlignHCenter

        font.pointSize: 8
        wrapMode: Text.WordWrap
      }
    }
  }
}
