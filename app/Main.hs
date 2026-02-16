module Main where

data List a
  = Empty
  | Cons a (List a)
  deriving (Show)

range :: Int -> Int -> List Int
range a b
  | a > b = Empty
  | otherwise = Cons a (range (a + 1) b)

-- double up: converts 1,2,3 to 1,1,2,2,3,3
doubleUp :: List a -> List a
doubleUp Empty = Empty
doubleUp (Cons x xs) = Cons x (Cons x (doubleUp xs))

-- take n from a list
taken :: Int -> List a -> List a
taken n _ | n <= 0 = Empty
taken _ Empty = Empty
taken n (Cons x xs) = Cons x (taken (n - 1) xs)

-- infinite custom list: x,x,x,...
repeatList :: a -> List a
repeatList x = Cons x (repeatList x)

main :: IO ()
main = do
  let fromRange = range 1 6
  putStrLn "range 1..6:"
  print fromRange

  let repeatedOnes :: List Int
      repeatedOnes = repeatList 1
  putStrLn "first 8 of repeatList 1:"
  print (taken 8 repeatedOnes)

  let oneAndTwo = range 1 2
  let doubleUpManyTimes = foldr (.) id (replicate 100000000 doubleUp)
  let expanded = doubleUpManyTimes oneAndTwo
  putStrLn "first 5 after doubling range 1..2 many times:"
  print (taken 5 expanded)
