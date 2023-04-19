import { useForm } from 'react-hook-form';
import { useAuth } from '@/hooks/useAuth';

type FormData = {
  username: string;
  password: string;
};

const Login = () => {
  const { login } = useAuth();
  const {
    register,
    handleSubmit,
  } = useForm<FormData>();

  return (
    <>
        <form onSubmit={handleSubmit(login)}>
          <input
            placeholder="Name"
            {...register('username', { required: true })}
          />
          <input
            placeholder="Password"
            {...register('password', { required: true })}
          />

          <input type="submit" />
        </form>
    </>
  );
};

export default Login;
