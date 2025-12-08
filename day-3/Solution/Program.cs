class Program
{
    static List<List<int>>? GetIntsFromFile(string path)
    {
        List<List<int>> ints = new List<List<int>>();
        try
        {
            if (!File.Exists(path))
                throw new FileNotFoundException("File does not exist @ " + path);

            foreach (string line in File.ReadAllLines(path))
            {
                List<int> lineInts = new List<int>();
                char[] charArr = line.ToCharArray();
                foreach (char c in charArr)
                {
                    lineInts.Add(int.Parse(c.ToString()));
                }
                ints.Add(lineInts);
            }
        }
        catch (Exception e)
        {
            Console.WriteLine($"Error: {e.Message}");
        }
        return ints;
    }

    static long PartOne(List<List<int>> batteryBanks)
    {
        long total = 0;

        foreach (List<int> battery in batteryBanks)
        {
            int maxCharge = 0;
            for (int i = battery.Count() - 2; i >= 0; i--)
            {
                for (int j = battery.Count() - 1; j >= 1; j--)
                {
                    if (j == i)
                        break;
                    if (maxCharge < (battery[i] * 10) + battery[j])
                        maxCharge = (battery[i] * 10) + battery[j];
                }
            }
            total += maxCharge;
        }
        return total;
    }

    static void PrintList(List<int> list)
    {
        foreach (int num in list)
            Console.Write(num);
        Console.WriteLine();
    }

    static ulong PartTwo(List<List<int>> batteryBanks)
    {
        ulong total = 0;

        foreach (List<int> batteryBank in batteryBanks)
        {
            Console.WriteLine("Processing New Battery...");
            Console.Write("\tInput Voltages : ");
            PrintList(batteryBank);
            Console.WriteLine("\tInput Length : " + batteryBank.Count);
            Console.WriteLine("\tVoltages To Remove : " + (batteryBank.Count - 12));

            Console.WriteLine("----");
        }
        return total;
    }

    public static void Main()
    {
        string path = "/home/penguin/source/advent-of-code-2025/day-3/example.txt";

        if (GetIntsFromFile(path) is List<List<int>> batteryBanks)
        {
            Console.WriteLine("[RESULT] PART-1 : " + PartOne(batteryBanks));
            Console.WriteLine("[RESULT] PART-2 : " + PartTwo(batteryBanks));
        }
    }
}