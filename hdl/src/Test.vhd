library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity Test is
  port (
    clk : in std_logic;
    led : out std_logic
  );
end Test;

architecture bhv of Test is
  signal counter : unsigned(24 downto 0) := (others => '0');
begin
  process(clk)
  begin
    if rising_edge(clk) then
      counter <= counter + 1;
    end if;
  end process;
  led <= counter(24);
end bhv;
