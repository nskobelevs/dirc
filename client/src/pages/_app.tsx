import Layout from '@/components/Layout';
import AuthContextProvider from '@/context/AuthContext';
import '@/styles/globals.css';
import type { AppProps } from 'next/app';
import { Toaster } from 'react-hot-toast';

const App = ({ Component, pageProps }: AppProps) => (
  <AuthContextProvider>
    <Layout>
      <Toaster />
      <Component {...pageProps} />
    </Layout>
  </AuthContextProvider>
);

export default App;
