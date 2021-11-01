(defn stack-new []
  '())

(defn stack-push [stack val]
  (conj stack val))

(defn stack-pop [stack]
  (rest stack))

(defn stack-top [stack]
  (first stack))
