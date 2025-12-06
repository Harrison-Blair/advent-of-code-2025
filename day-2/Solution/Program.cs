class Program
{
    static bool debug;

    static long[][]? GetRangesFromFile(string path)
    {
        long[][] ranges;
        try
        {
            if (File.Exists(path))
            {
                if (debug)
                {
                    Console.WriteLine("File Exists!");
                }
                //LINQ method chaning w/ lambda expressions
                ranges = Array.ConvertAll(
                    File.ReadAllText(path).Split(','),
                    s => Array.ConvertAll(
                        s.Split('-'),
                        long.Parse)
                );

                if (debug)
                {
                    Console.WriteLine("Ranges:");
                    Console.WriteLine("-" + string.Join("\n-", Array.ConvertAll(ranges, r => $"[{r[0]}-{r[1]}]")));
                }
            }
            else
            {
                Console.WriteLine($"File not found: {path}");
                return null;
            }
        }
        catch (Exception e)
        {
            Console.WriteLine($"Error reading file: {e.Message}");
            return null;
        }
        return ranges;
    }

    static long PartOne(long[][] ranges)
    {
        long total = 0;

        foreach (long[] range in ranges)
        {
            for (long number = range[0]; number <= range[1]; number++)
            {
                string numberString = number.ToString();
                if (numberString.Length % 2 != 0)
                {
                    if (debug)
                    {
                        Console.WriteLine("Skipping number with odd length: " + number);
                    }
                    continue;
                }

                int middle = numberString.Length / 2;
                string firstHalf = numberString.Substring(0, middle);
                string secondHalf = numberString.Substring(middle);

                if (debug)
                {
                    Console.WriteLine("1st Half String: " + firstHalf);
                    Console.WriteLine("2nd Half String: " + secondHalf);
                }

                if (firstHalf == secondHalf)
                {
                    total += number;

                    if (debug)
                    {
                        Console.WriteLine("Reflection found @ " + number);
                        Console.WriteLine("\t" + firstHalf + " | " + secondHalf);
                    }
                }
            }
        }
        return total;
    }

    static long PartTwo(long[][] ranges)
    {
        List<long> invalids = new List<long>();

        foreach (long[] range in ranges)
        {
            for (long number = range[0]; number <= range[1]; number++)
            {
                string numberString = number.ToString();
                for (int i = 1; i < numberString.Length; i++)
                {
                    string candidate = numberString.Substring(0, i);
                    if (numberString.Length % candidate.Length != 0) continue;

                    int repetitions = numberString.Length / candidate.Length;
                    string repetitionCandidate = string.Concat(Enumerable.Repeat(candidate, repetitions));
                    if (repetitionCandidate == numberString && !invalids.Contains(long.Parse(repetitionCandidate)))
                    {
                        invalids.Add(long.Parse(repetitionCandidate));
                    }
                }
            }
        }
        return invalids.Sum();
    }

    public static void Main(string[] args)
    {
        // I love ternary expressions!
        debug = (args.Length > 0 && args[0] == "debug") ? true : false;

        string path = "/home/penguin/source/advent-of-code-2025/day-2/input.txt";

        if (GetRangesFromFile(path) is long[][] ranges) //Assignment and null check
        {
            long partOneTotal = PartOne(ranges);
            long partTwoTotal = PartTwo(ranges);

            Console.WriteLine("[SOLUTION] Total for PART-1 is: " + partOneTotal);
            Console.WriteLine("[SOLUTION] Total for PART-2 is: " + partTwoTotal);
        }
    }
}