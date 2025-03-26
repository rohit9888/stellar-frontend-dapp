

import {
  allowAllModules,
  FREIGHTER_ID,
  StellarWalletsKit,
} from "@creit.tech/stellar-wallets-kit";
import BigNumber from "bignumber.js";
const SELECTED_WALLET_ID = "selectedWalletId";

function getSelectedWalletId() {
  return localStorage.getItem(SELECTED_WALLET_ID);
}

const kit = new StellarWalletsKit({
  modules: allowAllModules(),
  network: import.meta.env.PUBLIC_STELLAR_NETWORK_PASSPHRASE,
  // StellarWalletsKit forces you to specify a wallet, even if the user didn't
  // select one yet, so we default to Freighter.
  // We'll work around this later in `getPublicKey`.
  selectedWalletId: getSelectedWalletId() ?? FREIGHTER_ID,
});

export const signTransaction = kit.signTransaction.bind(kit);
  // async function CheckWalletConnect() {
  //   try {
  //     const isAppAvailable = await freighterApi.isConnected();
  //     if (!isAppAvailable.isConnected) {
  //       console.log("Freighter is not installed or not connected.");
  //       return null;
  //     }

  //     const isAppAllowed = await freighterApi.isAllowed();
  //     if (!isAppAllowed.isAllowed) {
  //       const allowed = await freighterApi.setAllowed();
  //       if (!allowed.isAllowed ) {
  //         console.log("User did not allow access.");
  //         return null;
  //       }
  //     }

     
  //   }catch (error) {
  //     console.error("Error connecting to Freighter:", error);
  //     return null;
  //   }
  // }
  export async function getPublicKey() {
    if (!getSelectedWalletId()) return null;
    const { address } = await kit.getAddress();
    return address;
  }
  
  export async function setWallet(walletId) {
    localStorage.setItem(SELECTED_WALLET_ID, walletId);
    kit.setWallet(walletId);
  }
  
  export async function disconnect(callback) {
    localStorage.removeItem('SELECTED_WALLET_ID');
    kit.disconnect();
    if (callback && typeof callback === 'function') {
      await callback();
    }
  }
  
  
  export async function connect(callback) {
    await kit.openModal({
      onWalletSelected: async (option) => {
        try {
          await setWallet(option.id);
          if (callback) await callback();
        } catch (e) {
          console.error(e);
        }
        return option.id;
      },
    });
  }

  export const xlmToStroop = (lumens) => {
    if (lumens instanceof BigNumber) {
      return lumens.times(1e7);
    }
    // round to nearest stroop
    return new BigNumber(Math.round(Number(lumens) * 1e7));
  };
  // async function getPublicKey() {
  //   try{
  //   const publicKey = await freighterApi.requestAccess();
  //   if (publicKey) {
      
  //     return publicKey.address;
  //   } else {
  //     console.log("Failed to retrieve public key.");
  //     return null;
  //   }
  // } catch (error) {
  //   console.error("Error to retrieve public :", error);
  //   return null;
  // }}
  export{
    // CheckWalletConnect,getPublicKey,
    
  }


