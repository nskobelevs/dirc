import AuthContextProvider from '@/context/AuthContext';
import '@/styles/globals.css';
import type { AppProps } from 'next/app';

const App = ({ Component, pageProps }: AppProps) => (
  <AuthContextProvider>
    <Component {...pageProps} />
  </AuthContextProvider>
);

export default App;
