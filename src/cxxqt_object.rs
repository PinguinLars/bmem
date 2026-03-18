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
#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        /// An alias to the QString type
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qset.h");
        type QSet_i32 = cxx_qt_lib::QSet<i32>;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(i32, number_of_cards)]
        #[qproperty(bool, your_turn)]
        #[qproperty(QSet_i32, cards_shown)]
        #[qproperty(bool, update)]
        #[namespace = "deck"]
        type Deck = super::DeckRust;

        #[qinvokable]
        #[cxx_name = "getCardText"]
        fn get_card_text(&self, index: i32) -> QString;

        #[qinvokable]
        #[cxx_name = "handleClickEvent"]
        fn handle_click_event(self: Pin<&mut Self>, index: i32);

        #[qinvokable]
        #[cxx_name = "isCardShown"]
        fn is_card_shown(&self, index: i32) -> bool;
    }
}

use cxx_qt_lib::{QSet, QString};
use core::pin::Pin;
use rand::{RngExt, seq::SliceRandom};

/// The Rust struct for the Deck Qt object
pub struct DeckRust {
    /// Length of `cards`
    /// Unique property because then qt can do updates
    number_of_cards: i32,
    /// The vector where all the cards are stored
    cards: Vec<Card>,
    /// If it is your (the players) turn
    your_turn: bool,
    /// A set of which card should be shown
    cards_shown: QSet<i32>,
    /// Property to force an update of text
    /// DATA IN IT IS NON CONSINTANT DON'T USE IT IN FUNCTIONS
    /// expect for rerenders
    update: bool,
}

impl qobject::Deck {
    /// Get the text that the card needs to hold as a Qt String
    pub fn get_card_text(&self, index: i32) -> QString {
        self.cards
            .get(index as usize)
            .expect("Index is out of range!")
            .string
            .clone()
    }

    /// Click handler for qml
    fn handle_click_event(mut self: Pin<&mut Self>, index: i32) {
        let mut cards_shown = self.cards_shown().clone();
        cards_shown.insert(index);
        self.as_mut().set_cards_shown(cards_shown);

        self.update_changed();
    }

    /// Helper function for cards in qml
    ///
    /// Wrapper around:
    /// ```
    /// self.cards_shown.contains(&index)
    /// ```
    fn is_card_shown(&self, index: i32) -> bool {
        self.cards_shown.contains(&index)
    }
}

impl Default for DeckRust {
    /// Constructor called by cxx-qt
    fn default() -> Self {
        let card_pairs = vec![
            Card::new_unique("Genotype".into(), "Informatie voor de erfelijke eigenschappen van een organisme".into()),
            Card::new_unique("Fenotype".into(), "Eigenschappen van een organisme, waaronder het uiterlijk".into()),
            Card::new_unique("Lichaamscel".into(), "Cellen waaruit je lichaam is opgebouwd".into()),
            Card::new_unique("Celdeling".into(), "Vorming van nieuwe cellen".into()),
            Card::new_unique("Dochtercel".into(), "Cel die ontstaat uit een moedercel tijdens celdeling".into()),
            Card::new_unique("DNA".into(), "Stof die informatie bevat voor erfelijke eigenschappen".into()),
            Card::new_unique("Gen".into(), "Stukjes DNA die samen de informatie bevatten voor een erfelijke eigenschap".into()),
            Card::new_unique("Chromosomen".into(), "Lange dunne draden in de celkern".into()),
            Card::new_unique("Geslachtscel".into(), "Cellen waarbij de chromosomen enkelvoudig voorkomen".into()),
            Card::new_unique("Meiose".into(), "Celdeling waarbij de chromosomen verdeeld worden over de dochtercellen (geslachtscellen)".into()),
            Card::new_unique("Chromosomen paar".into(), "Twee chromosomen die bestaan uit dezelfde genen vormen een paar".into()),
            Card::new_unique("Allelenpaar".into(), "Twee allelen van een gen".into()),
            Card::new_unique("Allel".into(), "Informatie in een gen".into()),
            Card::new_unique("Eiwit".into(), "Stof die voor een groot deel de kleur, vorm en werking van je lichaam regelt".into()),
            Card::new_unique("Dominant allel".into(), "Allel dat altijd tot uiting komt in het fenotype als er minimaal een is".into()),
            Card::new_unique("Recessief allel".into(), "Allel dat alleen tot uiting komt in het fenotype wanneer er geen dominant allel aanwezig is".into()),
            Card::new_unique("Homozygoot".into(), "Het allelenpaar voor een eigenschap bestaat uit twee gelijke allelen".into()),
            Card::new_unique("Heterozygoot".into(), "Het allelenpaar voor een eigenschap bestaat uit twee ongelijke allelen".into()),
            Card::new_unique("Base".into(), "A, T, C en G waar DNA uit is opgebouwd".into()),
            Card::new_unique("Basenpaar".into(), "Paar van de basen A-T of C-G".into()),
            Card::new_unique("Nucleotidenvolgorde".into(), "desp".into()),
        ];

        let mut cards: Vec<Card> = Vec::new();
        let mut rng = rand::rng();
        for i in card_pairs {
            cards.push(i.0);
            cards.push(i.1);
        }
        cards.shuffle(&mut rng);

        let number_of_cards = cards.len() as i32;

        Self {
            cards,
            number_of_cards,
            your_turn: rng.random_bool(0.5),
            cards_shown: QSet::default(),
            update: true,
        }
    }
}

/// The memory card
///
/// For matching pairs use Card::new_match
/// For unique pairs use Card::new_unique
#[derive(Debug)]
pub struct Card {
    /// String that is the value of this card
    string: QString,
    /// String of the card this card matches with (can be the same)
    matching_string: QString,
    /// If the pair is completed
    completed: bool,
}

impl Card {
    /// Generate new matching pair (same `string` and `matching_string` on both of them)
    pub fn new_match(string: String) -> (Self, Self) {
        let card = Card {
            string: (&string).into(),
            matching_string: (&string).into(),
            completed: false,
        };
        let matching_card = Card {
            string: (&string).into(),
            matching_string: (&string).into(),
            completed: false,
        };
        (card, matching_card)
    }

    /// Generate new unique pair of cards
    pub fn new_unique(string: String, matching_string: String) -> (Self, Self) {
        let card = Card {
            string: (&string).into(),
            matching_string: (&matching_string).into(),
            completed: false,
        };
        let matching_card = Card {
            string: (&matching_string).into(),
            matching_string: (&string).into(),
            completed: false,
        };
        (card, matching_card)
    }
}
