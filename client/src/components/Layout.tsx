import Head from 'next/head';
import type { PropsWithChildren } from 'react';

const Layout = ({ children }: PropsWithChildren) => (
  <>
    <Head>
      <title>dIRC</title>
      <meta
        name="description"
        content="Chat app made as part of COMP30220: Distributed Systems"
      />
      <meta name="viewport" content="width=device-width, initial-scale=1" />
      <link rel="icon" href="/favicon.ico" />
    </Head>

    <main>{children}</main>
  </>
);

export default Layout;
