<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create ShippingMethod</name>
   <tag></tag>
   <elementGuidId>122d9003-caea-477b-8082-d62ac326f543</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;name\&quot; : \&quot;DHL\&quot;,\n  \&quot;description\&quot; : \&quot;Standard delivery\&quot;,\n  \&quot;taxCategory\&quot; : {\n    \&quot;typeId\&quot; : \&quot;tax-category\&quot;,\n    \&quot;id\&quot; : \&quot;5a21f15b-34f8-4b7f-9407-d1eb82a73eba\&quot;\n  },\n  \&quot;zoneRates\&quot; : [ {\n    \&quot;zone\&quot; : {\n      \&quot;typeId\&quot; : \&quot;zone\&quot;,\n      \&quot;id\&quot; : \&quot;5cb532be-c680-43ab-ba14-b664bb03dc35\&quot;\n    },\n    \&quot;shippingRates\&quot; : [ {\n      \&quot;price\&quot; : {\n        \&quot;currencyCode\&quot; : \&quot;EUR\&quot;,\n        \&quot;centAmount\&quot; : 570\n      }\n    } ]\n  }, {\n    \&quot;zone\&quot; : {\n      \&quot;typeId\&quot; : \&quot;zone\&quot;,\n      \&quot;id\&quot; : \&quot;ebe01381-82be-4e63-9993-d1eb8f8e588b\&quot;\n    },\n    \&quot;shippingRates\&quot; : [ {\n      \&quot;price\&quot; : {\n        \&quot;currencyCode\&quot; : \&quot;USD\&quot;,\n        \&quot;centAmount\&quot; : 990\n      }\n    } ]\n  } ],\n  \&quot;isDefault\&quot; : false\n}&quot;,
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
      <webElementGuid>5fef3eac-eec3-47a8-8af4-f6165c6e2164</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>c0f5ad6b-fbe8-43f1-9704-7305797b647a</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/shipping-methods</restUrl>
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
      <id>8af58ddd-929b-4eec-bf60-681e934d7759</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>5bd888f6-9ced-49e0-82f5-4d5053096cee</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
