import {
  Card,
  Flex,
  Icon,
  Stack,
  Tab,
  TabList,
  Tabs,
  Text,
  Tooltip,
} from "@chakra-ui/react";
import { useRouter } from "next/router";
import { useTranslation } from "react-i18next";
import {
  LuBox,
  LuCircleUserRound,
  LuCompass,
  LuSettings,
  LuZap,
} from "react-icons/lu";
import { TitleShort } from "@/components/logo-title";
import { useLauncherConfig } from "@/contexts/config";

const HeadNavBar = () => {
  const router = useRouter();
  const { t } = useTranslation();
  const { config } = useLauncherConfig();
  const primaryColor = config.appearance.theme.primaryColor;
  const isSimplified = config.appearance.theme.headNavStyle !== "standard";
  const isVertical = config.appearance.theme.headNavStyle === "simplified-left";

  const navList = [
    { icon: LuZap, label: "launch", path: "/launch" },
    { icon: LuBox, label: "games", path: "/games" },
    { icon: LuCircleUserRound, label: "accounts", path: "/accounts" },
    ...(config.general.optionalFunctions.discover
      ? [{ icon: LuCompass, label: "discover", path: "/discover" }]
      : []),
    { icon: LuSettings, label: "settings", path: "/settings" },
  ];

  const selectedIndex = navList.findIndex((item) =>
    router.pathname.startsWith(item.path)
  );

  return (
    <Flex justify="center" p={4}>
      <Card
        className="content-blur-bg"
        px={isVertical ? 2 : 8}
        py={isVertical ? 8 : 2}
      >
        <Stack
          spacing={4}
          direction={isVertical ? "column" : "row"}
          alignItems="center"
        >
          <TitleShort />
          <Tabs
            variant="soft-rounded"
            size="sm"
            orientation={isVertical ? "vertical" : "horizontal"}
            colorScheme={primaryColor}
            index={selectedIndex}
            onChange={(index) => {
              router.push(navList[index].path);
            }}
          >
            <TabList>
              {navList.map((item, index) => (
                <Tooltip
                  key={item.path}
                  label={t(`HeadNavBar.navList.${item.label}`)}
                  placement="bottom"
                  isDisabled={!isSimplified || selectedIndex === index}
                >
                  <Tab fontWeight={selectedIndex === index ? "600" : "normal"}>
                    <Stack
                      spacing={2}
                      direction={isVertical ? "column" : "row"}
                      alignItems="center"
                    >
                      <Icon as={item.icon} />
                      {(!isSimplified || selectedIndex === index) && (
                        <Text
                          style={{
                            writingMode: isVertical
                              ? "vertical-rl"
                              : "horizontal-tb",
                          }}
                        >
                          {t(`HeadNavBar.navList.${item.label}`)}
                        </Text>
                      )}
                    </Stack>
                  </Tab>
                </Tooltip>
              ))}
            </TabList>
          </Tabs>
        </Stack>
      </Card>
    </Flex>
  );
};

export default HeadNavBar;
