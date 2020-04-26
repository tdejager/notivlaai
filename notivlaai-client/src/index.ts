/**
 * Simple prime number sieve
 */
const multiples = (n: number, limit: number): number[] => {
  const result: number[] = [];
  let i = 2;

  while (i * n <= limit) {
    result.push(i * n);
    i += 1;
  }

  return result;
};

const sqrtInt = (n: number): number => Math.trunc(Math.sqrt(n));

const primes = (limit: number): number[] => {
  const sieve = Array(limit + 1);

  for (let i = 0; i <= limit; i += 1) {
    sieve[i] = false;
  }

  for (let i = 2; i <= sqrtInt(limit); i += 1) {
    multiples(i, limit).forEach(value => {
      sieve[value] = true;
    });
  }

  const result: number[] = [];

  for (let i = 2; i <= limit; i += 1) {
    if (!sieve[i]) {
      result.push(i);
    }
  }

  return result;
};

export default primes;
