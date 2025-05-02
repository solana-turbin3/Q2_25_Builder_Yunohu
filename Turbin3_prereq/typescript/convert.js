const bs58 = require('bs58').default;
const prompt = require('prompt-sync')();

const base58 = prompt("Enter your base58 private key: ");
const walletArray = bs58.decode(base58);
console.log("64-byte array:", Array.from(walletArray));

const base58Converted = bs58.encode(walletArray);
console.log("Base58 again:", base58Converted);
// tool to convert phantom or web3 wallet private key to the b64 array format which should be placed in wallet.json file for the transactions