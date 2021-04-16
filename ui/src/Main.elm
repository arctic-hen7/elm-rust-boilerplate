module Main exposing (..)

import Element exposing (..)

main =
    layout [] view

view : Element.Element msg
view =
    column
        [
            width (px 200)
            , height (px 200)
            , centerX
            , centerY
        ]
        [
            el [ centerX, centerY ] (text "Hello from Elm!")
        ]
