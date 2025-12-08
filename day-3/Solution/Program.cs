class Program
{
    static List<List<long>>? GetlongsFromFile(string path)
    {
        List<List<long>> longs = new List<List<long>>();
        try
        {
            if (!File.Exists(path))
                throw new FileNotFoundException("File does not exist @ " + path);

            foreach (string line in File.ReadAllLines(path))
            {
                List<long> linelongs = new List<long>();
                char[] charArr = line.ToCharArray();
                foreach (char c in charArr)
                {
                    linelongs.Add(c - '0');
                }
                longs.Add(linelongs);
            }
        }
        catch (Exception e)
        {
            Console.WriteLine($"Error: {e.Message}");
        }
        return longs;
    }

    static long PartOne(List<List<long>> batteryBanks)
    {
        long total = 0;

        foreach (List<long> battery in batteryBanks)
        {
            long maxCharge = 0;
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

    static List<long> FindLeading(List<long> batteryBank)
    {
        int largestIndex = 0;
        for (int i = 0; i < 4; i++)
        {
            if (batteryBank[i] > batteryBank[largestIndex])
                largestIndex = i;
        }
        List<long> longs = batteryBank.Slice(largestIndex, batteryBank.Count - largestIndex);

        return longs;
    }

    static long ListToNumber(List<long> list)
    {
        long number = 0;
        for (int i = 0; i < list.Count; i++)
        {
            number += (long)Math.Pow(10, list.Count - i - 1) * list[i];
        }
        return number;
    }

    static long PartTwo(List<List<long>> batteryBanks) {
        long total = 0;

        foreach (List<long> batteryBank in batteryBanks)
        {
            Console.WriteLine(ListToNumber(batteryBank));
            Console.WriteLine(ListToNumber(FindLeading(batteryBank)));
        }
        return total;
    }

    public static void Main()
    {
        string path = "/home/penguin/source/advent-of-code-2025/day-3/example.txt";

        if (GetlongsFromFile(path) is List<List<long>> batteryBanks)
        {
            Console.WriteLine("[RESULT] PART-1 : " + PartOne(batteryBanks));
            Console.WriteLine("[RESULT] PART-2 : " + PartTwo(batteryBanks));
        }
    }
}