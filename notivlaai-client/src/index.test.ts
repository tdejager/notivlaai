import primes from './index';

describe('Prime number component', () => {
  it('says hello', () => {
    expect.assertions(1);
    const primeNumbers = primes(11);
    expect(primeNumbers).toStrictEqual([2, 3, 5, 7, 11]);
  });
});
