# Fetches all slither link puzzles from https://puzsq.logicpuzzle.app/.
ENDPOINT=https://asia-northeast1-puzzle-square.cloudfunctions.net/loadThumbnailData
curl ${ENDPOINT} --request POST --data '{"data":{"query":{"sortMethod":{"type":"time","order":"DESC"},"page":1,"perPage":10000,"kind":1,"variation":0},"isMyData":false}}' -H 'content-type: application/json' | jq --raw-output '.result.rows[].url' >puzzles.txt
