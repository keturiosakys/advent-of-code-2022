(ns day1
  (:require [clojure.string :as str]))

(defn parse-int [number-string]
  (try (Integer/parseInt number-string)
       (catch Exception e nil)))

(defn largest-load [inputs]
  (let [parsed-inputs (slurp inputs)]
    (->> (str/split parsed-inputs #"\n\n")
         (mapv #(str/split % #"\n"))
         (mapv #(->> %
                     (mapv parse-int)
                     (reduce +)))
         (apply max))))


(defn top-three [inputs]
  (let [parsed-inputs (slurp inputs)]
    (->> (str/split parsed-inputs #"\n\n")
         (mapv #(str/split % #"\n"))
         (mapv #(->> %
                     (mapv parse-int)
                     (reduce +)))
         (sort >)
         (take 3)
         (reduce +))))


(largest-load "aoc-01/calories_input.txt")
(top-three "aoc-01/calories_input.txt")



