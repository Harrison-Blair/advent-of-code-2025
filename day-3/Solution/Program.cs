class Program
{
    static List<List<int>>? GetIntsFromFile(string path)
    {
        List<List<int>> ints = new List<List<int>>();
        try
        {
            if (!File.Exists(path))
                throw new FileNotFoundException("File does not exist @ " + path);

            foreach (string line in File.ReadAllLines(path)) {
                List<int> lineInts = new List<int>();
                char[] charArr = line.ToCharArray();
                foreach (char c in charArr) {
                    lineInts.Add(c - '0');
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
    
    static int PartOne(List<List<int>> batteryBanks) {
        int total = 0;

        foreach (List<int> battery in batteryBanks) {
            
        }
        return total;
    }

    public static void Main()
    {
        string path = "/home/penguin/source/advent-of-code-2025/day-3/example.txt";
        
        if (GetIntsFromFile(path) is List<List<int>> batteryBanks) {
            Console.WriteLine(PartOne(batteryBanks));
        }
    }
}