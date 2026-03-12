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

          text: "Your score: 21/21\nComputer score: 0/21"
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

          text: "Turn: computer" // or you
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
        text: "Biologie memory (bmem) is free software licensed under the GPL license"

        Layout.fillWidth: true
        horizontalAlignment: Text.AlignHCenter

        font.pointSize: 8
        wrapMode: Text.WordWrap
      }
    }
  }
}
