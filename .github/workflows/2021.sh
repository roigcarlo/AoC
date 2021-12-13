for d in ${GITHUB_WORKSPACE}/*/ ; do
    for p in ${d}code* ; do
        python3.10 ${p} ${d}/data
    done
    echo "\n"
done