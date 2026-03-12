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

  RowLayout {
    anchors.fill: parent
    spacing: 0

    GridLayout {
      rows: 6 
      columns: 7
      Layout.fillHeight: true
      Layout.preferredWidth: parent.width * 0.8
      columnSpacing: 5
      rowSpacing: 5

      Repeater {
        model: 42

        Button {
          Layout.fillWidth: true
          Layout.fillHeight: true
          text: index + 1

          onClicked: {
            console.log("Button " + (index + 1) + " clicked")
          }
        }
      }
    }

    ColumnLayout {
      Layout.fillHeight: true
      Layout.preferredWidth: parent.width * 0.2

      Rectangle {
        Layout.preferredWidth: scoreText.implicitWidth
        Layout.preferredHeight: scoreText.implicitHeight
        Layout.fillWidth: true
        // Layout.fillHeight: true

        Text {
          id: scoreText
          
          anchors.fill: parent
          text: "Your score: 21/21\nComputer score: 0/21"
        }
      }

      Rectangle {
        Layout.preferredWidth: turnText.implicitWidth
        Layout.preferredHeight: turnText.implicitHeight
        Layout.fillWidth: true
        // Layout.fillHeight: true

        Text {
          id: turnText
          anchors.fill: parent
          text: "Turn: computer" // or you
        }
      }

      Rectangle {
        Layout.preferredWidth: diffText.implicitWidth
        Layout.preferredHeight: diffText.implicitHeight
        Layout.fillWidth: true
        // Layout.fillHeight: true

        Text {
          id: diffText
          anchors.fill: parent
          text: "Moeilijkheids: 50%"
        }
      }

      Text {
        text: "Biologie memory (bmem) is free software licensed under the GPL license"

        Layout.fillWidth: true
        horizontalAlignment: Text.AlignHCenter

        font.pointSize: 8
        wrapMode: Text.WordWrap
      }
    }
  }
}
