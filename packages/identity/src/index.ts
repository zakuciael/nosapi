// Przykładowe użycie (a w zasadzie jak ja to widzę):


const gfapi = GameforgeAPI(...)
await gfapi.auth('login', 'password')
const accoutsList = await gfapi.accounts()
const token = await gfapi.getPlayToken('accountId')

// getPlayToken w środku
// pobiera clientVersion (jest api do tego)
// sendGameStartedEvent
// create blackbox
// sendIovation
// getGa