// esse arquivo será utilizado como base para a criação de classes e seus efeitos
// ou seja, teremos efeitos e atributos de classes genéricos, para utilização de outros sistemas.

// devemos criar um component com movimento, pontos de ação, pontos de "efry", alcance, altura

use bevy::prelude::Component;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable)]
pub struct BasicsAttributes {
    pub hpp: (i32, i32),
    pub movp: (i32, i32),
    pub actp: (i32, i32),
    pub efyp: (i32, i32),
    pub ranp: (i32, i32),
    pub height: (i32, i32),
    pub area: (i32, i32),
}

impl BasicsAttributes {
    fn hpp(&self) -> (i32, i32) {
        self.hpp
    }
    fn movp(&self) -> (i32, i32) {
        self.movp
    }
    fn actp(&self) -> (i32, i32) {
        self.actp
    }
    fn efyp(&self) -> (i32, i32) {
        self.efyp
    }
    fn ranp(&self) -> (i32, i32) {
        self.ranp
    }
    pub fn height(&self) -> (i32, i32) {
        self.height
    }
    pub fn area(&self) -> (i32, i32) {
        self.area
    }

    fn set_hpp(&mut self, x: (i32, i32)) {
        self.hpp = x;
    }
    fn set_movp(&mut self, x: (i32, i32)) {
        self.movp = x;
    }
    fn set_actp(&mut self, x: (i32, i32)) {
        self.actp = x;
    }
    fn set_efyp(&mut self, x: (i32, i32)) {
        self.efyp = x;
    }
    fn set_ranp(&mut self, x: (i32, i32)) {
        self.ranp = x;
    }
    fn set_height(&mut self, x: (i32, i32)) {
        self.height = x;
    }
    fn set_area(&mut self, x: (i32, i32)) {
        self.area = x;
    }
}


#[derive(Inspectable,Default,Copy,Clone)]
pub enum Team {
    #[default]
    Allies,
    Enemies,
    Passives,
    Other,
}

impl Default for BasicsAttributes {
    fn default() -> Self {
        Self {
            hpp: (0,0),
            movp: (0,0),
            actp: (0,0),
            efyp: (0,0),
            ranp: (0,0),
            height: (0,0),
            area: (0,0),
        }
    }
}

enum DefaultBorder {
    Movep(i32),
    Actp(i32),
    Efyp(i32),
    Range(i32),
}
