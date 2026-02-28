library ieee;
use ieee.std_logic_1164.all;

entity Top is
  port (
    clk_25mhz : in std_logic;
    led       : out std_logic_vector(7 downto 0)
  );
end Top;

architecture rtl of Top is
  signal blink : std_logic := '0';
begin
  u_test : entity work.Test
    generic map (
      BITS => 25
    )
    port map (
      clk => clk_25mhz,
      led => blink
    );

  led <= (6 downto 0 => '0', 7 => blink);
end rtl;
