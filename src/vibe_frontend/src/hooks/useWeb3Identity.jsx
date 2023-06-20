import { useContext, createContext } from 'react';

const Web3Context = createContext(false);

const useWeb3Identity = () => {
  const accessWeb3 = useContext(Web3Context);

  return accessWeb3;
};

export default useWeb3Identity;
