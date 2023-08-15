module Main exposing (main)

import Browser
import Html as H exposing (Html)
import Html.Attributes as HA
-- import Html.Events as HE


main : Program () Model Msg
main =
    Browser.sandbox
        { init = init
        , view = view
        , update = update
        }



-- MODEL


type alias Calendar =
    { minutes : Int, lat : Float, long : Float, prepTime : Int }


type alias Model =
    { calendar : Maybe Calendar }


init : Model
init =
    { calendar = Nothing }



-- UPDATE


type Msg
    = UpdateCalendar Calendar


update : Msg -> Model -> Model
update msg model =
    case msg of
        UpdateCalendar calendar ->
            { model | calendar = Just calendar }



-- VIEW


view : Model -> Html Msg
view m =
    H.div []
        [ H.h1 [] [ H.text "Mincha Minder" ]
        , H.p [] [ H.text "Fill in the details for your mincha reminder" ]
        , H.form [] -- TODO add an onSubmit in here somehow
            [ H.label [] [ H.text "Minutes before / after sunset" ]
            , H.input
                [ HA.name "minutes"
                , HA.type_ "number"
                , HA.value
                    (case m.calendar of
                        Nothing ->
                            ""

                        Just c ->
                            c.minutes |> String.fromInt
                    )
                ]
                []
            , H.br [] []
            , H.label [] [ H.text "Latitude" ]
            , H.input
                [ HA.name "lat"
                , HA.type_ "number"
                , HA.value
                    (case m.calendar of
                        Nothing ->
                            ""

                        Just c ->
                            c.lat |> String.fromFloat
                    )
                ]
                []
            , H.br [] []
            , H.label [] [ H.text "Longitude" ]
            , H.input
                [ HA.name "long"
                , HA.type_ "number"
                , HA.value
                    (case m.calendar of
                        Nothing ->
                            ""

                        Just c ->
                            c.long |> String.fromFloat
                    )
                ]
                []
            , H.br [] []
            , H.label [] [ H.text "Prep time (how long before do you want to be reminded?" ]
            , H.input
                [ HA.name "prep_time"
                , HA.value
                    (case m.calendar of
                        Nothing ->
                            ""

                        Just c ->
                            c.prepTime |> String.fromInt
                    )
                ]
                []
            ]
        ]
