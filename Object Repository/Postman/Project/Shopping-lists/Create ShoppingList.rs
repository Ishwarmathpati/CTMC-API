<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create ShoppingList</name>
   <tag></tag>
   <elementGuidId>5ffc5fb4-774b-4494-b561-fff5838b8d0f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;name\&quot; : {\n    \&quot;en\&quot; : \&quot;My shopping list\&quot;\n  },\n  \&quot;slug\&quot; : {\n    \&quot;en\&quot; : \&quot;my-shopping-list\&quot;\n  },\n  \&quot;customer\&quot; : {\n    \&quot;typeId\&quot; : \&quot;customer\&quot;,\n    \&quot;id\&quot; : \&quot;e73cd97f-846e-44a0-b418-3ed044a8e398\&quot;\n  },\n  \&quot;key\&quot; : \&quot;my-shopping-list\&quot;,\n  \&quot;deleteDaysAfterLastModification\&quot; : 100,\n  \&quot;lineItems\&quot; : [ {\n    \&quot;sku\&quot; : \&quot;product-variant-sku\&quot;,\n    \&quot;quantity\&quot; : 5\n  }, {\n    \&quot;productId\&quot; : \&quot;0e131f46-8d1a-4761-9c83-b45ab5d3501e\&quot;,\n    \&quot;variantId\&quot; : 2\n  } ],\n  \&quot;textLineItems\&quot; : [ {\n    \&quot;name\&quot; : {\n      \&quot;en\&quot; : \&quot;My shopping list item\&quot;\n    },\n    \&quot;description\&quot; : {\n      \&quot;en\&quot; : \&quot;This is a good gift idea\&quot;\n    },\n    \&quot;quantity\&quot; : 5\n  } ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer {{ctp_access_token}}</value>
      <webElementGuid>9361a3ca-ab7e-4d13-8fcb-6440c760e59b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>244915dc-1918-4d8d-8fbf-88f2dfb78988</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/shopping-lists</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>5fa41bc7-e6cd-4ad4-8a62-7e755945c3f8</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>3375223e-7e9a-4059-b25c-6f43d1dd9a89</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
