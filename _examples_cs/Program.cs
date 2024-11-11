void OpenNonexistentFile() {
    try 
    {
        File.Open("non_existent", FileMode.Open);
    }
    catch (Exception e) {
        Console.WriteLine($"{e}");
    }
}

OpenNonexistentFile();