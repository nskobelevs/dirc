const API_URL = process.env.DIRC_API_URL ?? 'localhost';
const API_PORT = process.env.DIRC_API_PORT ?? '8080';

const api = async (url: string, init?: RequestInit) => {
  const response = await fetch(`http://${API_URL}:${API_PORT}/${url}`, init);

  if (!response.ok) {
    throw new Error(response.statusText);
  }

  return response.json();
};

export const get = async <T>(url: string, config?: RequestInit): Promise<T> => {
  const init = {
    ...config,
    method: 'GET',
  };

  return api(url, init);
};

export const post = async <TResponse, TBody>(
  url: string,
  body: TBody,
  config?: RequestInit,
): Promise<TResponse> => {
  const init = {
    ...config,
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(body),
  };

  return api(url, init);
};

export const put = async <TResponse, TBody>(
  url: string,
  body: TBody,
  config?: RequestInit,
): Promise<TResponse> => {
  const init = {
    ...config,
    method: 'PUT',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(body),
  };

  return api(url, init);
};

export const del = async <T>(url: string, config?: RequestInit): Promise<T> => {
  const init = {
    ...config,
    method: 'DELETE',
  };

  return api(url, init);
};
