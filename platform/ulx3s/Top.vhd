library ieee;
use ieee.std_logic_1164.all;

entity Top is
  port (
    clk_25mhz : in std_logic;
    rx        : in std_logic;
    led       : out std_logic_vector(7 downto 0)
  );
end Top;

architecture rtl of Top is
  signal uart_data  : std_logic_vector(7 downto 0) := (others => '0');
  signal uart_valid : std_logic := '0';
  signal led_reg    : std_logic_vector(7 downto 0) := (others => '0');
begin
  u_uart : entity work.Uart
    port map (
      clk        => clk_25mhz,
      rx         => rx,
      data_valid => uart_valid,
      data       => uart_data
    );

  process(clk_25mhz)
  begin
    if rising_edge(clk_25mhz) then
      if uart_valid = '1' then
        led_reg <= uart_data;
      end if;
    end if;
  end process;

  led <= led_reg;
end rtl;
