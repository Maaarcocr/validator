if [[ "$OSTYPE" == "linux-gnu" ]]; then 
    ./bin/formula_creator
    ./prover
    ./bin/checker
elif [[ "$OSTYPE" == "darwin"* ]]; then 
    ./bin/formula_creator_mac
    ./prover
    ./bin/checker_mac
fi
