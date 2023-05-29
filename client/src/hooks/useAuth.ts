import { useRouter } from 'next/router';
import { useEffect } from 'react';
import { useCookies } from 'react-cookie';

import { get, post, put } from '@/api';
import type {
  AuthenticateResponse,
  LoginBody,
  LoginResponse,
  RegisterBody,
  RegisterResponse,
} from '@/api/types/auth';
import { useAuthContext } from '@/context/AuthContext';

type AuthForm = {
    username: string;
    password: string;
}

export const useAuth = () => {
  const [user, setUser] = useAuthContext();

  const router = useRouter();
  const [cookies, setCookie, removeCookie] = useCookies(['token']);

  const authenticate = async (token: string) => {
    const { username } = await get<AuthenticateResponse>(
      'auth/authenticate',
      {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      },
    );

    return username;
  };

  useEffect(() => {
    const { token } = cookies;

    const handleAuthenticate = async () => {
      const username = await authenticate(token);
      setUser({ username, token });
    };

    if (token) {
      handleAuthenticate().catch(() => {
        // TODO handle error
      });

      return;
    }

    const { pathname } = router;
    if (!['/login', '/register'].includes(pathname)) router.push('/login');
  }, [cookies, router, setUser]);

  const register = async ({ username, password }: AuthForm) => {
    const { token } = await put<RegisterResponse, RegisterBody>(
      'auth/register',
      {
        username,
        password,
      },
    );

    setCookie('token', token);
  };

  const login = async ({ username, password }: AuthForm) => {
    const { token } = await post<LoginResponse, LoginBody>('auth/login', {
      username,
      password,
    });

    setCookie('token', token);
  };

  const logout = () => {
    setUser(null);
    removeCookie('token');
    router.push('/login');
  };

  return {
    user,
    register,
    login,
    logout,
  };
};
