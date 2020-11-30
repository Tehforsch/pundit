mkdir -p benchmark 
for i in $(seq -w 1 10000); do
    echo "#+TITLE: note$i" > "benchmark/note$i.org"
    if [[ $i != "00001" ]]; then
        echo "[[file:note00001.org][note1]]" >> "benchmark/note$i.org"
    fi
done
