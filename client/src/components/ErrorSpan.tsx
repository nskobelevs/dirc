import { MultipleFieldErrors } from 'react-hook-form';

type ErrorSpanProps = {
  message: string;
  messages?: MultipleFieldErrors | undefined;
};

const style = {
  color: 'hsl(var(--er))',
  fontSize: '0.875rem',
};

const ErrorSpan = ({ message }: ErrorSpanProps) => (
  <p style={style}>{message}</p>
);

export default ErrorSpan;
