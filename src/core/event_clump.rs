use events::{GameFromMainGameCity, GameFromMainGameMap, GameFromMainGamePauseMenu,
             GameFromMainMenu, GameToMainGameCity, GameToMainGameMap, GameToMainGamePauseMenu,
             GameToMainMenu, MainFromControl, MainFromGame, MainFromRender, MainToControl,
             MainToGame, MainToRender};
use utils::DuoChannel;

pub fn make_event_clumps<ID>() -> (FrontEventClump<ID>, BackEventClump<ID>)
where
    ID: Send + Eq,
{
    let (front_main_x_control, back_main_x_control) = DuoChannel::new_both();
    let (front_main_x_game, back_main_x_game) = DuoChannel::new_both();
    let (front_main_x_render, back_main_x_render) = DuoChannel::new_both();
    let (front_game_x_main_menu, back_game_x_main_menu) = DuoChannel::new_both();
    let (front_game_x_main_game_pause_menu, back_game_x_main_game_pause_menu) =
        DuoChannel::new_both();
    let (front_game_x_main_game_map, back_game_x_main_game_map) = DuoChannel::new_both();
    let (front_game_x_main_game_city, back_game_x_main_game_city) = DuoChannel::new_both();

    (
        FrontEventClump::new(front_main_x_control, front_main_x_game, front_main_x_render),
        BackEventClump::new(
            back_main_x_control,
            back_main_x_game,
            back_main_x_render,
            front_game_x_main_menu,
            back_game_x_main_menu,
            front_game_x_main_game_pause_menu,
            back_game_x_main_game_pause_menu,
            front_game_x_main_game_map,
            back_game_x_main_game_map,
            front_game_x_main_game_city,
            back_game_x_main_game_city,
        ),
    )
}

pub struct BackEventClump<ID>
where
    ID: Send + Eq,
{
    main_x_control: Option<DuoChannel<MainFromControl, MainToControl>>,
    main_x_game: Option<DuoChannel<MainFromGame, MainToGame>>,
    main_x_render: Option<DuoChannel<MainFromRender<ID>, MainToRender<ID>>>,
    front_game_x_main_menu: Option<DuoChannel<GameToMainMenu, GameFromMainMenu>>,
    back_game_x_main_menu: Option<DuoChannel<GameFromMainMenu, GameToMainMenu>>,
    front_game_x_main_game_pause_menu:
        Option<DuoChannel<GameToMainGamePauseMenu, GameFromMainGamePauseMenu>>,
    back_game_x_main_game_pause_menu:
        Option<DuoChannel<GameFromMainGamePauseMenu, GameToMainGamePauseMenu>>,
    front_game_x_main_game_map: Option<DuoChannel<GameToMainGameMap, GameFromMainGameMap>>,
    back_game_x_main_game_map: Option<DuoChannel<GameFromMainGameMap, GameToMainGameMap>>,
    front_game_x_main_game_city: Option<DuoChannel<GameToMainGameCity, GameFromMainGameCity>>,
    back_game_x_main_game_city: Option<DuoChannel<GameFromMainGameCity, GameToMainGameCity>>,
}

impl<ID> BackEventClump<ID>
where
    ID: Send + Eq,
{
    fn new(
        main_x_control: DuoChannel<MainFromControl, MainToControl>,
        main_x_game: DuoChannel<MainFromGame, MainToGame>,
        main_x_render: DuoChannel<MainFromRender<ID>, MainToRender<ID>>,
        front_game_x_main_menu: DuoChannel<GameToMainMenu, GameFromMainMenu>,
        back_game_x_main_menu: DuoChannel<GameFromMainMenu, GameToMainMenu>,
        front_game_x_main_game_pause_menu: DuoChannel<
            GameToMainGamePauseMenu,
            GameFromMainGamePauseMenu,
        >,
        back_game_x_main_game_pause_menu: DuoChannel<
            GameFromMainGamePauseMenu,
            GameToMainGamePauseMenu,
        >,
        front_game_x_main_game_map: DuoChannel<GameToMainGameMap, GameFromMainGameMap>,
        back_game_x_main_game_map: DuoChannel<GameFromMainGameMap, GameToMainGameMap>,
        front_game_x_main_game_city: DuoChannel<GameToMainGameCity, GameFromMainGameCity>,
        back_game_x_main_game_city: DuoChannel<GameFromMainGameCity, GameToMainGameCity>,
    ) -> BackEventClump<ID> {
        BackEventClump {
            main_x_control: Some(main_x_control),
            main_x_game: Some(main_x_game),
            main_x_render: Some(main_x_render),
            front_game_x_main_menu: Some(front_game_x_main_menu),
            back_game_x_main_menu: Some(back_game_x_main_menu),
            front_game_x_main_game_pause_menu: Some(front_game_x_main_game_pause_menu),
            back_game_x_main_game_pause_menu: Some(back_game_x_main_game_pause_menu),
            front_game_x_main_game_map: Some(front_game_x_main_game_map),
            back_game_x_main_game_map: Some(back_game_x_main_game_map),
            front_game_x_main_game_city: Some(front_game_x_main_game_city),
            back_game_x_main_game_city: Some(back_game_x_main_game_city),
        }
    }

    pub fn take_main_x_control(&mut self) -> Option<DuoChannel<MainFromControl, MainToControl>> {
        self.main_x_control.take()
    }

    pub fn take_main_x_game(&mut self) -> Option<DuoChannel<MainFromGame, MainToGame>> {
        self.main_x_game.take()
    }

    pub fn take_main_x_render(
        &mut self,
    ) -> Option<DuoChannel<MainFromRender<ID>, MainToRender<ID>>> {
        self.main_x_render.take()
    }

    pub fn take_front_game_x_main_menu(
        &mut self,
    ) -> Option<DuoChannel<GameToMainMenu, GameFromMainMenu>> {
        self.front_game_x_main_menu.take()
    }

    pub fn take_back_game_x_main_menu(
        &mut self,
    ) -> Option<DuoChannel<GameFromMainMenu, GameToMainMenu>> {
        self.back_game_x_main_menu.take()
    }

    pub fn take_front_game_x_main_game_pause_menu(
        &mut self,
    ) -> Option<DuoChannel<GameToMainGamePauseMenu, GameFromMainGamePauseMenu>> {
        self.front_game_x_main_game_pause_menu.take()
    }

    pub fn take_back_game_x_main_game_pause_menu(
        &mut self,
    ) -> Option<DuoChannel<GameFromMainGamePauseMenu, GameToMainGamePauseMenu>> {
        self.back_game_x_main_game_pause_menu.take()
    }

    pub fn take_front_game_x_main_game_map(
        &mut self,
    ) -> Option<DuoChannel<GameToMainGameMap, GameFromMainGameMap>> {
        self.front_game_x_main_game_map.take()
    }

    pub fn take_back_game_x_main_game_map(
        &mut self,
    ) -> Option<DuoChannel<GameFromMainGameMap, GameToMainGameMap>> {
        self.back_game_x_main_game_map.take()
    }

    pub fn take_front_game_x_main_game_city(
        &mut self,
    ) -> Option<DuoChannel<GameToMainGameCity, GameFromMainGameCity>> {
        self.front_game_x_main_game_city.take()
    }

    pub fn take_back_game_x_main_game_city(
        &mut self,
    ) -> Option<DuoChannel<GameFromMainGameCity, GameToMainGameCity>> {
        self.back_game_x_main_game_city.take()
    }
}

pub struct FrontEventClump<ID>
where
    ID: Send + Eq,
{
    control: Option<DuoChannel<MainToControl, MainFromControl>>,
    game: Option<DuoChannel<MainToGame, MainFromGame>>,
    render: Option<DuoChannel<MainToRender<ID>, MainFromRender<ID>>>,
}

impl<ID> FrontEventClump<ID>
where
    ID: Send + Eq,
{
    fn new(
        control: DuoChannel<MainToControl, MainFromControl>,
        game: DuoChannel<MainToGame, MainFromGame>,
        render: DuoChannel<MainToRender<ID>, MainFromRender<ID>>,
    ) -> FrontEventClump<ID> {
        FrontEventClump {
            control: Some(control),
            game: Some(game),
            render: Some(render),
        }
    }

    pub fn get_mut_control(&mut self) -> Option<&mut DuoChannel<MainToControl, MainFromControl>> {
        self.control.as_mut()
    }

    pub fn get_mut_game(&mut self) -> Option<&mut DuoChannel<MainToGame, MainFromGame>> {
        self.game.as_mut()
    }

    pub fn get_mut_render(
        &mut self,
    ) -> Option<&mut DuoChannel<MainToRender<ID>, MainFromRender<ID>>> {
        self.render.as_mut()
    }
}
