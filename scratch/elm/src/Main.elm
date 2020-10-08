module Main exposing (..)

-- Functions


isEven : Int -> Bool
isEven x =
    modBy x 2 == 0


wrap : Int -> a -> ( Int, a )
wrap id value =
    ( id, value )


workingWithFunctions : ()
workingWithFunctions =
    let
        n =
            10

        even_result =
            isEven n

        square =
            \x -> x * x

        n_squared =
            square n

        numbers =
            [ 0, 1, 2 ]

        numbers_squared =
            List.map square numbers
    in
    ()


allEven : List Int -> Bool
allEven xs =
    List.all isEven xs


multiplyAllBy : List Int -> Int -> List Int
multiplyAllBy xs amount =
    List.map (\x -> x * amount) xs



-- Control Flow


controlFlow : ()
controlFlow =
    let
        cents =
            25

        coin =
            if cents == 1 then
                "penny"

            else if cents == 5 then
                "nickel"

            else
                ""

        coin_case =
            case cents of
                1 ->
                    "penny"

                5 ->
                    "nickel"

                _ ->
                    ""
    in
    ()



-- Records, Union Types


type alias Point =
    { x : Int
    , y : Int
    }


type Direction
    = Up
    | Down
    | Left
    | Right


step : Point -> Direction -> Point
step position to =
    let
        { x, y } =
            position
    in
    case to of
        Up ->
            { position | y = y + 1 }

        Down ->
            { position | y = y - 1 }

        Left ->
            { position | x = x - 1 }

        Right ->
            { position | x = x + 1 }


type Msg
    = Start
    | Move Direction
    | Run Int Direction


type alias Model =
    { isPlaying : Bool
    , position : Point
    }


update : Model -> Msg -> Model
update model msg =
    case msg of
        Start ->
            { model | isPlaying = True }

        Move direction ->
            { model | position = step model.position direction }

        Run speed direction ->
            model



-- Generics


type alias QueryResults a =
    { page : Int
    , next : Bool
    , results : List a
    }



-- Maybe


stepIfVertical : Point -> Direction -> Maybe Point
stepIfVertical position to =
    let
        { x, y } =
            position
    in
    case to of
        Up ->
            Just { position | y = y + 1 }

        Down ->
            Just { position | y = y - 1 }

        _ ->
            Nothing


workingWithMaybe : ()
workingWithMaybe =
    let
        maybe =
            Just "10"

        otherMaybeCase =
            case maybe of
                Just s ->
                    Just (String.length s)

                Nothing ->
                    Nothing

        otherMaybeMap =
            Maybe.map String.length maybe

        otherMaybeAndThen =
            Maybe.andThen (\s -> String.toInt s) maybe
    in
    ()
