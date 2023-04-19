import Layout from '@/components/Layout';
import AuthContextProvider from '@/context/AuthContext';
import '@/styles/globals.css';
import type { AppProps } from 'next/app';

const App = ({ Component, pageProps }: AppProps) => (
  <AuthContextProvider>
    <Layout>
      <Component {...pageProps} />
    </Layout>
  </AuthContextProvider>
);

export default App;
