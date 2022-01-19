using System;
using System.Runtime.InteropServices;
using System.Windows;
using System.Windows.Interop;

namespace cliplacer
{
    class Program
    {
        [DllImport("user32", SetLastError = true)]
        private static extern bool RegisterHotKey(IntPtr hWnd, int id, uint fsModifiers, uint vk);

        // [DllImport("user32", SetLastError = true)]
        // private static extern bool UnregisterHotKey(IntPtr hWnd, int id);

        [DllImport("user32.dll")]
        private static extern int GetMessage(out MSG lpMsg, IntPtr hWnd, uint wMsgFilterMin,
            uint wMsgFilterMax);

        private static readonly uint WM_HOTKEY = 0x0312;
        private static readonly uint MOD_ALT = 0x0001;
        private static readonly uint MOD_CTRL = 0x0002;
        private static readonly uint MOD_NOREPEAT = 0x4000;
        private static readonly uint VK_C = 0x43;

        [STAThread]
        static void Main(string[] args)
        {
            if (!RegisterHotKey(IntPtr.Zero, 1, MOD_ALT | MOD_CTRL | MOD_NOREPEAT, VK_C))
            {
                Environment.Exit(1);
            }

            Console.WriteLine("Register Ctrl+Alt+C successfully");
            
            MSG msg;
            while (GetMessage(out msg, IntPtr.Zero, 0, 0) != 0)
            {
                if (msg.message != WM_HOTKEY)
                    continue;

                if (!Clipboard.ContainsText()) 
                    continue;
                
                var newContent = Clipboard.GetText().Replace(Environment.NewLine, "");
                Console.WriteLine($"New content: {newContent}");
                Clipboard.SetText(newContent);
            }
        }
    }
}
