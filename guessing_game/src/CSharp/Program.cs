using System;

namespace CSharp
{
    class Program
    {
        static void Main(string[] args) {
            Console.WriteLine("Guess the number!");

            uint secret_number = (uint)new Random().Next(1, 101);

            do {
                Console.WriteLine("Please input your guess.");

                if (!uint.TryParse(Console.ReadLine().Trim(), out uint guess)) {
                    continue;
                }

                Console.WriteLine($"You guessed: {guess}");

                switch (guess.CompareTo(secret_number)) {
                    case -1: Console.WriteLine("Too small!"); 
                        break;
                    case 1: Console.WriteLine("Too big!"); 
                        break;
                    case 0: Console.WriteLine("You win!"); 
                        return;
                }
            } while (true);
        }
    }
}
