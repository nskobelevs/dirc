import { useForm } from 'react-hook-form';
import { useAuth } from '@/hooks/useAuth';
import Link from 'next/link';
import { ErrorMessage } from '@hookform/error-message';
import cn from 'classnames';
import ErrorSpan from '@/components/ErrorSpan';

type FormData = {
  username: string;
  password: string;
  confirm: string;
};

const Register = () => {
  const { register: signUp } = useAuth();
  const {
    register,
    watch,
    handleSubmit,
    formState: { errors },
  } = useForm<FormData>();

  return (
    <>
      <div
        style={{
          background:
            'linear-gradient(-45deg, #ee7752, #e73c7e, #23a6d5, #23d5ab)',
          backgroundSize: '400% 400%',
          animation: 'gradient 15s ease infinite',
        }}
        className="flex flex-col w-screen h-screen justify-center"
      >
        <form
          className="container mx-auto py-12 px-24 w-[32rem] h-[36rem] bg-gray-50 flex flex-col items-center justify-center gap-y-3 border border-gray-300 rounded-xl"
          onSubmit={handleSubmit(signUp)}
        >
          <h1 className="text-4xl justify-self-start font-extrabold mb-10">
            Welcome to dIRC
          </h1>

          <div className="w-full">
            <div className="w-full min-h-[70px]">
              <input
                className={cn(
                  'input input-bordered w-full',
                  errors.username && 'input-error',
                )}
                placeholder="Username"
                {...register('username', { required: 'Username is required' })}
              />

              <ErrorMessage
                errors={errors}
                name="username"
                render={ErrorSpan}
              />
            </div>

            <div className="w-full min-h-[70px]">
              <input
                type="password"
                className={cn(
                  'input input-bordered w-full',
                  errors.password && 'input-error',
                )}
                placeholder="Password"
                {...register('password', { required: 'Password is required' })}
              />

              <ErrorMessage
                errors={errors}
                name="password"
                render={ErrorSpan}
              />
            </div>

            <div className="w-full min-h-[70px]">
              <input
                type="password"
                className={cn(
                  'input input-bordered w-full',
                  errors.confirm && 'input-error',
                )}
                placeholder="Confirm password"
                {...register('confirm', {
                  validate: (value) => {
                    if (watch('password') !== value) {
                      return 'Passwords do not match';
                    }
                    return true;
                  },
                })}
              />

              <ErrorMessage errors={errors} name="confirm" render={ErrorSpan} />
            </div>
          </div>

          <hr className="h-0.5 my-4 bg-gray-300 w-96 rounded border-0" />

          <div className="w-full flex flex-col items-center">
            <button className="btn btn-primary btn-wide" type="submit">
              Sign up
            </button>
            <span className="text-gray-500 mt-3">
              Already have an account?{' '}
              <Link href="/login" className="link link-accent">
                Log in
              </Link>
            </span>
          </div>
        </form>
      </div>
    </>
  );
};

export default Register;
