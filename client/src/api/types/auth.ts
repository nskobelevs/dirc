export type RegisterBody = {
  username: string;
  password: string;
};

export type RegisterResponse = {
  username: string;
  token: string;
};

export type LoginBody = {
  username: string;
  password: string;
};

export type LoginResponse = {
  username: string;
  token: string;
};

export type AuthenticateResponse = {
  username: string;
};

export type LogoutResponse = null;
