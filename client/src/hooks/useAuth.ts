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

export const useAuth = () => {
  const [user, setUser] = useAuthContext();

  const router = useRouter();
  const [cookies, setCookie, removeCookie] = useCookies(['token']);

  const authenticate = async (token: string) => {
    const { username } = await get<AuthenticateResponse>(
      'authenticate',
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
      setUser({ username });
    };

    if (token) {
      handleAuthenticate().catch(() => {
        // TODO handle error
      });

      return;
    }

    if (router.pathname !== '/login') router.push('/login');
  }, [cookies, router, setUser]);

  const register = async (username: string, password: string) => {
    const { token } = await put<RegisterResponse, RegisterBody>(
      'register',
      {
        username,
        password,
      },
    );

    setCookie('token', token);
  };

  const login = async (username: string, password: string) => {
    const { token } = await post<LoginResponse, LoginBody>('login', {
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
