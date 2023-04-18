import type { Dispatch, SetStateAction } from 'react';
import {
  PropsWithChildren, createContext, useContext, useState,
} from 'react';

import { User } from '@/types/auth';

type AuthContext = [User | null, Dispatch<SetStateAction<User | null>>] | null;

const AuthContext = createContext<AuthContext>(null);

const AuthContextProvider = ({ children }: PropsWithChildren) => {
  const [user, setUser] = useState<User | null>(null);

  return (
    <AuthContext.Provider value={[user, setUser]}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuthContext = () => {
  const context = useContext(AuthContext);

  if (context === null) {
    throw new Error('useAuthContext must be used within an AuthContextProvider');
  }

  return context;
};

export default AuthContextProvider;
